mod config;
mod ffmpeg;
mod api;
mod context;

// use std::convert::Infallible;
// 为了让一个地方注解不出问题
use hyper::body::HttpBody;
use crate::context::Context;
use crate::config::Config;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, StatusCode, Method};
use hyper::service::{make_service_fn, service_fn};
use futures::TryStreamExt as _;
use crate::api::MakeApiSvc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {


    let ctx = Context::from_config(Config::new());

    let config  = ctx.cfg();
    // 绑定端口 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // service 将会被使用在每一个connection中，所以需要创建一个
    // let make_svc = make_service_fn(|_conn| async {
    //     // service_fn将自己的程序转化成一个service
    //     Ok::<_, hyper::Error>(service_fn(api::handler::get_stream))
    // });

    let server = Server::bind(&addr).serve(MakeApiSvc::new(config));

     // And now add a graceful shutdown signal...
    //  let graceful = server.with_graceful_shutdown(shutdown_signal());

    //  if let Err(e) = graceful.await {
    //     eprintln!("server error: {}", e);
    // }

    println!("listening on http://{}", addr);

    // 如果使用graceful的话，最后不能再使用server、

    server.await?;

    Ok(())
}

// 创建服务
// service明确了如何应对访问
async fn hello_world(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // new 的response将会默认拥有200 OK的标志头
    // Body 可以看出它是由静态字符串组成的，
    // 并且自动为我们添加一个 Content-Length 头
    // Ok(Response::new("hello world".into()))
    println!("{:?}", req);

    let mut response = Response::new(Body::empty());

    // 匹配req的类型
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        // into_body: Consumes the request, returning just the body.
        // body: 
        // A stream of Bytes, used when receiving bodies.
        // A good default HttpBody to use in many applications.
        // Note: To read the full body, use body::to_bytes or body::aggregate.
        (&Method::POST, "/echo") => {
            println!("接收到了访问");
            Some("car").unwrap_or("bike");
            // let body = req.into_body();
            
            // *response.body_mut() = Body::from("an anwser");
            *response.body_mut() = req.into_body();
        },
        // Yet another route inside our match block...
        (&Method::POST, "/echo/uppercase") => {
        // 这里同样还没有测试，3.10日搞
        // This is actually a new `futures::Stream`...
            let mapping = req
                .into_body()
                // map_ok: Wraps the current stream in a new stream which maps the success value using the provided closure.
                .map_ok(|chunk| {
                    chunk.iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Vec<u8>>()
                });

            // Use `Body::wrap_stream` to convert it to a `Body`...
            *response.body_mut() = Body::wrap_stream(mapping);
        },
                // Yet another route inside our match block...
                // 这里的作用是buffer request
        (&Method::POST, "/echo/reverse") => {
            // 之前error的问题在于hyper::error类型不匹配，即返回值应当是hyper::errror
            // 在编写调用许多返回 Result 类型的函数的代码时，错误处理可能会很乏味。 问号运算符 ? 隐藏了一些在调用堆栈中向上传播错误的样板。
            
            // Protect our server from massive bodies.
            // 无法真正在数据传入时对其进行流式处理，因为我们需要在响应之前找到结束点。
            // 注意不要在没有最大界限的情况下进行缓冲。
            // size_hint: 返回流剩余长度的界限。当流的确切剩余长度已知时，上限将被设置并且将等于下限。
            // upper: 返回body剩余数据的上限，Option 枚举的包含none和some
            // 之前直接写的body，改为into后无措，如果为into_body还是有错
            // body(): Returns a reference to the associated HTTP body.
            // body_mut(): Returns a mutable reference to the associated HTTP body.
            let upper = req.body().size_hint().upper().unwrap_or(u64::MAX);
            if upper > 1024 * 64 {
                let mut resp = Response::new(Body::from("Body too big"));
                *resp.status_mut() = hyper::StatusCode::PAYLOAD_TOO_LARGE;
                return Ok(resp);
            }

            let full_body = hyper::body::to_bytes(req.into_body()).await?;

            // Await the full body to be concatenated into a single `Bytes`...
            
            

            // Iterate the full body in reverse order and collect into a new Vec.
            let reversed = full_body.iter()
                .rev()
                .cloned()
                .collect::<Vec<u8>>();

            *response.body_mut() = reversed.into();
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)

    // Body 实现了来自 futures 的 Stream 特性，在数据传入时产生一堆 Bytess。Bytes 只是来自 hyper 的一种方便的类型，它代表一堆字节。 它可以很容易地转换成其他典型的字节容器。
}


async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
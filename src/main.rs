mod config;
mod ffmpeg;
mod api;

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, StatusCode, Method};
use hyper::service::{make_service_fn, service_fn};
use futures::TryStreamExt as _;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 绑定端口 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // service 将会被使用在每一个connection中，所以需要创建一个
    let make_svc = make_service_fn(|_conn| async {
        // service_fn将自己的程序转化成一个service
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("listening on http://{}", addr);

    server.await?;

    Ok(())
}

// 创建服务
// service明确了如何应对访问
async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // new 的response将会默认拥有200 OK的标志头
    // Body 可以看出它是由静态字符串组成的，
    // 并且自动为我们添加一个 Content-Length 头
    // Ok(Response::new("hello world".into()))

    let mut response = Response::new(Body::empty());

    // 匹配req的类型
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        // 这里还没有测试，明天搞
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        },
        // Yet another route inside our match block...
        (&Method::POST, "/echo/uppercase") => {
        // 这里同样还没有测试，3.10日搞
        // This is actually a new `futures::Stream`...
            let mapping = req
                .into_body()
                .map_ok(|chunk| {
                    chunk.iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Vec<u8>>()
                });

            // Use `Body::wrap_stream` to convert it to a `Body`...
            *response.body_mut() = Body::wrap_stream(mapping);
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)

    // Body 实现了来自 futures 的 Stream 特性，在数据传入时产生一堆 Bytess。Bytes 只是来自 hyper 的一种方便的类型，它代表一堆字节。 它可以很容易地转换成其他典型的字节容器。
}

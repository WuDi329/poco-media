mod config;
mod ffmpeg;
mod api;

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

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
async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // new 的response将会默认拥有200 OK的标志头
    // Body 可以看出它是由静态字符串组成的，
    // 并且自动为我们添加一个 Content-Length 头
    Ok(Response::new("hello world".into()))
}

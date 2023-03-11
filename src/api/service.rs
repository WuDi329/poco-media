use hyper::{service::Service, Request, Body, Response};
use super::router::{Router, Route};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use super::router::Handler;
use super::handler;
use crate::config::SharedCfg;

// Pin是一个这样的智能指针，他内部包裹了另外一个指针P，并且只要P指针指向的内容（我们称为T）没有实现Unpin，则可以保证T永远不会被移动（move）。
// futurepin包含了一个指针，该指针的位置不会变
// 这个指针是一个动态类型
// Rust 中的 Futures 类似于 Javascript 中的promise[1]，它们是对 Rust 中并发原语的强大抽象。
// 这也是通往async/await[2]的基石，async/await 能够让用户像写同步代码一样来写异步代码。
type FuturePin<T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'static>>;


pub struct ApiService {
    config: SharedCfg,
    router: Router
}

impl ApiService {
    fn new(config: SharedCfg) -> ApiService {
        let mut router = Router::new();
        // router.add(Route::get(r"/movies/(\d+)").name("get_movie"));
        // router.add(Route::get("/movies/").name("get_movies"));
        // 将path为stream和get_stream方法添加到router
        router.add(Route::get(r"/stream/(\d+)").name("get_stream"));
        ApiService { config, router }
    }
}
impl Service<Request<Body>> for ApiService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = FuturePin<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    // 在调试程序时，这里是第一个进入的程序
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        println!("{:?}", req);
        // 这里调用router的is_match方法，返回了相关的Route
        if let Some(route) = self.router.is_match(req.uri().path()) {
            let res: Handler = match route.name.as_ref() {
                // 返回值是pin<Box<get_movies>>，调用了handler::get_movies
                // "get_movies" => Box::pin(handler::get_movies(self.db.clone())),
                // "get_movie" => {
                //     // 获取url携带的其他参数
                //     let id = route.params[0].parse().unwrap();
                //     Box::pin(handler::get_movie(self.db.clone(), id))
                // }
                // 如果是get_stream 就匹配到handler中的service方法
                "get_stream" => {
                    // // 获取url携带的其他参数
                    let id = route.params[0].parse().unwrap();
                    Box::pin(handler::get_stream(
                        // 将config中的config进行复制
                        self.config.clone(),
                        id,
                    ))
                }
                // 最后直接返回未实现
                _ => unimplemented!(),
            };
            return res;
        }

        Box::pin(async { Ok(Response::builder().body(Body::from("Not Found")).unwrap()) })
    }
}

pub struct MakeApiSvc {
    config: SharedCfg,
}

// 为MakeApiSvc实现new方法
impl MakeApiSvc {
    pub fn new(config: SharedCfg) -> MakeApiSvc {
        MakeApiSvc { config }
    }
}

// 为MakeApiSvc实现了Service接口
impl<T> Service<T> for MakeApiSvc {
    // MakeApiSvc的response是一个实际的ApiService
    type Response = ApiService;
    // hyper是一个rust 的 http 库
    type Error = hyper::Error;
    // futurePin
    // FuturePin<Result<Self::Response, Self::Error>>
    type Future = FuturePin<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn call(&mut self, _: T) -> Self::Future {
        // 获取MakeApiSvc的config和db，这里直接使用clone方法
        let config = self.config.clone();
        // let db = self.db.clone();

        // routes

        //  在这里通过一个service初始化了另外一个 service ，也就是ApiService
        let fut = async move { Ok(ApiService::new(config)) };
        Box::pin(fut)
    }
}
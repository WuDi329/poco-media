use hyper::{Method, Response, Body, Error};
use super::path::Path;
use std::future::Future;
use std::pin::Pin;


pub type Handler =
    Pin<Box<dyn Future<Output = Result<Response<Body>, Error>> + Send + Sync + 'static>>;
pub struct RouteBuilder {
    route: Route,
}

impl RouteBuilder {
    pub fn new(route: Route) -> RouteBuilder{
        RouteBuilder { route }
    }

//  初始赋值只包括了method和path，name应当是后面补充
//  在命名name之后，直接返回router
    pub fn name(mut self, name: &str) -> Route{
        self.route.name = name.to_owned();
        self.route
    }
}

#[derive(Clone)]
pub struct Route {
    pub method: Method,

    pub path: Path,

    pub name: String,

    pub params: Vec<String>
}

impl Route {
    // 返回一个RouteBuilder
    // 默认传入参数，get
    pub fn get(path: &str) -> RouteBuilder {
        Route::from(Method::GET, path)
    }

    fn from(method: Method, path: &str) -> RouteBuilder {
        RouteBuilder::new(Route { 
            method, 
            path: Path::new(path), 
            name: "".to_owned(), 
            params: Vec::new(),
         })
    }
}

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes:Vec::new() }
    }

    pub fn add(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn is_match(&mut self, path: &str) -> Option<Route> {
        for route in self.routes.iter_mut() {
            if route.path.matcher.is_match(path) {
                // regex::Regex::captures 是 Rust 中的一个方法，用于从一个字符串中提取匹配的子串。
                // 这个方法返回一个 Option 类型，表示字符串是否与正则表达式匹配。
                // 如果匹配成功，则返回一个 Captures 对象，它包含所有捕获的子串。通常，可以将这个方法用于解析文本或提取特定的信息。
                // 在这里就是提取出url的后面的子串
                let caps = route.path.matcher.captures(path).unwrap();
                if caps.len() > 1 {
                    route.params.push(caps[1].to_owned());
                }
                return Some(route.clone());
            }
        }
        None
    }
}
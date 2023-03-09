use hyper::{header, Body, Response, StatusCode};
use crate::config::{Config, self};
use crate::ffmpeg::FFmpeg;
use std::sync::Arc;
use serde_json::{to_string};

// 宏看起来和函数很像，只不过名称末尾有一个感叹号 ! 。宏并不产生函数调用，而是展开成源码，并和程序的其余部分一起被编译。
// 这一部分定义了json 宏，配置允许跨域
macro_rules! json {
    ($x:expr) => {
        match serde_json::to_string($x) {
            Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from(json))
                .unwrap(),
            Err(_) => Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("INTERNAL_SERVER_ERROR".into())
                .unwrap(),
        }
    };
}

pub async fn get_stream(
    // db目前没有作用先删除
    // db: SharedDb,
    config: Arc<Config>,
    id: i32,
) -> Result<Response<Body>, hyper::Error> {
    let config = Arc::new(config.ffmpeg.clone());

    // 创建ffmpeg，使用相应config进行初始化
    let ffmpeg = FFmpeg::new(config);

    // Runtime::channel()创建带有关联的发送者。当想要从另一个线程流式传输chunk时很有用。
    let (tx, body) = Body::channel();

    // 开启新的线程调用ffmpeg下的transcode方法
    tokio::spawn(async move {
        ffmpeg.transcode("h264.mkv", tx).await;
    });

    // 构建给前端的返回值，返回值包含了body，body在主线程中接收来自新线程转码后的结果
    let resp = Response::builder()
    .header("Content-Type", "video/mp4")
    .header("Content-Disposition", "inline")
    .header("Content-Transfer-Enconding", "binary")
    .body(body)
    .unwrap();

    Ok(resp)
}


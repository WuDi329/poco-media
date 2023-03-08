use crate::config::FFmpegConfig;
use hyper::body::Bytes;
use hyper::body::Sender;
use std::io::{BufReader, Read};
use std::process::{Command, Stdio};
use std::sync::Arc;

struct ArgBuilder<'a> {
    args: Vec<&'a str>,
}

impl<'a> ArgBuilder<'a> {
    // 使用具体的参数填充转码模板
    fn with(mut self, name: &str, val: &'a str) -> ArgBuilder<'a> {
        let search = format!("%{}", name);
        if let Some(arg) = self.args.iter_mut().find(|a| **a == search) {
            *arg = val;
        } else {
            println!("[W]: ffmpeg argument {} could not be set.", name);
        }
        self
    }

    //build()返回ArgBuilder的所有参数
    fn build(self) -> Vec<&'a str> {
        self.args
    }
}

pub struct FFmpeg {
    // FFmpeg的唯一属性config是FFmpegConfig,它会在transcode.toml中被读取
    config: Arc<FFmpegConfig>,
}

impl FFmpeg {
    pub fn new(config: Arc<FFmpegConfig>) -> FFmpeg {
        FFmpeg { config }
    }

    // 具体的参数要改写
    pub async fn transcode(&self, file: &str, mut sender: Sender) {
        let args = self
        .build_args()
        .with()
    }

    // 构造config
    fn build_args(&self) -> ArgBuilder<'_> {
        // 这里采用了配置文件中*的配置
        let args = self
            .config
            .codecs
            .get("*")
            .unwrap()
            .args
            .iter()
            .map(|a| a.as_str())
            .collect::<Vec<&str>>();
        ArgBuilder { args }

    }
}
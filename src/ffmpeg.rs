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
        .with("i",file)
        // 这里需要根据传入视频的自身设置确定其分辨率
        .with("s", "1920x1080")
        // 这里同样需要根据传入视频的bitrate确定转码后的bitreate
        .with("b", "1000000")
        .build();

        // 此处执行完成会获得完整的转码结果
        let mut cmd = Command::new(&self.config.bin)
            .args(&args)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut buf: [u8; 65536] = [0; 65536];

        let mut stdout = BufReader::new(cmd.stdout.as_mut().unwrap());

        while let Ok(()) = stdout.read_exact(&mut buf) {
            let b = Bytes::copy_from_slice(&buf);
            sender.send_data(b).await.unwrap();
            buf = [0; 65536];
        }

        let status = cmd.wait();
        println!("exited with status {:?}", status);

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
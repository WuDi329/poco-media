use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::sync::Arc;
use std::collections::HashMap;

pub type SharedCfg = Arc<Config>;

#[derive(Debug, Clone, Deserialize)]
pub struct CodecConfig {
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FFmpegConfig {
    pub bin: String,
    pub codecs: HashMap<String, CodecConfig>,
}

// 目前只有一个有关ffmpeg的配置，但是有可能会增多
#[derive(Debug, Deserialize)]
pub struct Config {
    pub ffmpeg: FFmpegConfig,
}

impl Config {
    pub fn new() -> SharedCfg {
        Config::default().into_shared()
    }

    // 读取文件生成配置
    pub fn from_file<P: AsRef<Path>>(file: P) -> io::Result<Config> {
        let mut contents = String::new();
        let mut file = File::open(file)?;
        // 读取file的内容到contents中
        file.read_to_string(&mut contents)?;
        // 通过toml::from_str把transcode.toml的字符串反序列化成类型
        let config: Config = toml::from_str(&contents)?;
        println!("{:?}", config);
        Ok(config)  

    }

    // 数据封装成Arc，用来在多线程中共享
    pub fn into_shared(self) -> SharedCfg {
        Arc::new(self)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config::from_file("transcode.toml").unwrap()
    }
}
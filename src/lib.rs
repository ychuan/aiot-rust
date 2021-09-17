#![allow(dead_code)]
#![allow(unused)]

//! Rust Link SDK，提供阿里云物联网平台的设备端 Rust 开发工具包（非官方）。
//!
//! This document won't be translated to English because "Aliyun IoT Platform" only has it's Chinese version.
//!
//! 遵循阿里云物联网平台定义的 [Alink 协议](https://help.aliyun.com/document_detail/90459.html)。

pub use alink::ThreeTuple;
pub use dm::{DataModel, DataModelOptions};
pub use dm::msg::{DataModelMsg, MsgEnum};
pub use dm::recv::{DataModelRecv, RecvEnum};
pub use dynregmq::{DynamicRegister, DynamicRegisterResult};
pub use http::Http;
pub use logpost::LogPost;
pub use mqtt::{DeviceAuthInfo, MqttClient, MqttInstance};
pub use ntp::NtpService;
pub use ra::RemoteAccess;
pub use util::error::{Error, Result};

pub mod util;
pub mod alink;
pub mod mqtt;
pub mod http;
pub mod ra;
pub mod dm;
pub mod dynregmq;
pub mod ntp;
pub mod logpost;
pub mod ota;
pub mod http_downloader;
pub mod subdev;
pub mod tag;
pub mod shadow;
pub mod remote_config;
pub mod bootstrap;

#[async_trait::async_trait]
pub(crate) trait Executor {
    async fn execute(&self, topic: &str, payload: &[u8]) -> Result<()>;
}

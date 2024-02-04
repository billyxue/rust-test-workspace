use serde::Deserialize;
use config::{Config, Environment, File};

//为了能够从各种格式反序列化这些结构，我们必须将宏添加Deserialize 到结构中。
//我还添加了Default宏，以便能够实例化这些结构，而无需为所有字段指定值。
//该Debug宏也很方便，因此我们可以稍后轻松记录配置内容。

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Logging {
    pub log_level: Option<String>,
    // Option<>基本值，如字符串、数字、布尔标志。这样我们就可以轻松地用默认值替换缺失值：
    // pub log_level: String,
}

//let log_level = Settings.Logging.log_level.unwrap_or("info");

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct ConfigInfo {
    pub location: Option<String>,
    pub env_prefix: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Settings {
    #[serde(default)]
    pub config: ConfigInfo,
    // 结构，我更喜欢使用默认值，因此总是为我们构建一个空的配置结构：
    #[serde(default)]
    pub database: Database,
    #[serde(default)]
    pub logging: Logging,
}
//
// 主Settings结构将包含数据库和日志记录配置。
//
impl Settings {
    pub fn new(location: &str, env_prefix: &str) -> anyhow::Result<Self> {
        let s = Config::builder()
            .add_source(File::with_name(location))
            .add_source(
                Environment::with_prefix(env_prefix)
                    .separator("__")
                    .prefix_separator("__"),
            )
            .set_override("config.location", location)?
            .set_override("config.env_prefix", env_prefix)?
            .build()?;

        let settings = s.try_deserialize()?;

        Ok(settings)
    }
}

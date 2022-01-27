use std::time::Duration;

use envconfig::Envconfig;

#[derive(Envconfig, Default)]
pub struct Config {
    #[envconfig(from = "YM_PROMETHEUS", default = "0.0.0.0:9090")]
    pub prometheus_endpoint: String,

    #[envconfig(from = "YM_NEOSEGMENT", default = "0.0.0.0:3000")]
    pub neosegment_endpoint: String,

    #[envconfig(from = "YM_PROMETHEUS_QUERY")]
    pub prometheus_query: String,

    #[envconfig(from = "YM_NEOSEGMENT_FORMAT")]
    pub neosegment_format: String,

    #[envconfig(from = "YM_RESOLUTION_MS", default = "1000")]
    pub resolution_ms: u64,
}

impl Config {
    pub fn resolution(&self) -> Duration {
        Duration::from_millis(self.resolution_ms)
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    // #[async_std::test]
    // async fn test_Config_parse_credentials_is_base64() {
    //     let mut conf = Config::default();
    //     let (v1, v2) = ("id|user|password|clientid|secret", "abcd");
    //     conf.api_credentials = format!(
    //         "netatmo:{},someotherservice:{}",
    //         base64::encode(v1),
    //         base64::encode(v2)
    //     );
    //     let expected = {
    //         let mut h = HashMap::new();
    //         h.insert("netatmo".to_string(), v1.to_string());
    //         h.insert("someotherservice".to_string(), v2.to_string());
    //         h
    //     };
    //     assert_eq!(conf.parsed_credentials().await, expected);
    // }
}

mod comms;
mod config;
mod sink;
mod source;
use anyhow::Result;
use config::Config;

use async_std::stream::StreamExt;
use envconfig::Envconfig;
use sink::neosegment::NeosegmentSink;
use source::prometheus::PrometheusSource;
use surf::Url;
use xactor::*;

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();
    let config: Config = Config::init_from_env()?;

    // Start actor and get its address
    let _neoseg = NeosegmentSink::new(
        Url::parse(&config.neosegment_endpoint)?,
        &config.neosegment_format,
        Box::new(|c| {
            let (r, g, b) = match c.round() as i64 {
                0..=10 => (0, 255, 0),
                11..=20 => (255, 255, 0),
                21..=30 => (255, 155, 0),
                _ => (255, 0, 0),
            };
            (r << 16) | (g << 8) | b
        }),
    )
    .start()
    .await?;

    let _prometheus = PrometheusSource::new(
        Url::parse(&config.prometheus_endpoint)?,
        &config.prometheus_query,
        config.resolution(),
    )
    .start()
    .await?;

    async_std::stream::pending::<()>().next().await;
    Ok(())
}

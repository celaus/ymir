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
            let s = 255.0 / 50.0; // max
            let g = 255_f64.min(0_f64.max(2.0 * (255.0 - s * c))) as u32;
            let r = 255_f64.min(2.0 * s * c) as u32;
            // 0xwwggrrbb
            let w = 0;
            // let wind = c.round() as i32 * 5;
            // let g = 0.max(255 - wind) as u32;
            // let r = 255.min(wind) as u32;
            let b = 0 as u32;

            // let (r, g, b) = match c.round() as i64 {
            //     0..=10 => (255, 0, 0),
            //     11..=20 => (255, 255, 0),
            //     21..=30 => (155, 255, 0),
            //     _ => (0, 255, 0),
            // };
            (w << 24) | (g << 16) | (r << 8) | b
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

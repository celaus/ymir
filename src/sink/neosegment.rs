use std::sync::Arc;

use crate::comms::QueryResult;
use dyn_fmt::AsStrFormatExt;
use itertools::Itertools;
use log::error;
use surf::{http::convert::Serialize, Url};
use xactor::*;

#[derive(Serialize, Debug, Clone)]
struct NeoSegmentParams {
    text: String,
    colors: String,
    timeout: usize,
}

pub struct NeosegmentSink {
    url: Url,
    format: String,
    colors_fn: Arc<Box<dyn Fn(f64) -> u32 + 'static + Sync + Send>>,
}

impl NeosegmentSink {
    pub fn new<I, U>(
        url: U,
        format: I,
        colors_fn: Box<dyn Fn(f64) -> u32 + 'static + Sync + Send>,
    ) -> Self
    where
        I: Into<String>,
        U: Into<Url>,
    {
        NeosegmentSink {
            url: url.into(),
            format: format.into(),
            colors_fn: Arc::new(colors_fn),
        }
    }
}

#[async_trait::async_trait]
impl Actor for NeosegmentSink {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        ctx.subscribe::<QueryResult>().await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Handler<QueryResult> for NeosegmentSink {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: QueryResult) {
        match msg {
            QueryResult::Scalar { value, unit } => {
                let txt: String = self.format.format(&[&value.to_string(), &unit]);
                let color = (self.colors_fn)(value);
                let colors: String = format!("[{}]", txt.chars().map(|_| color).join(","));
                let params = NeoSegmentParams {
                    text: txt,
                    colors,
                    timeout: 0,
                };
                if let Ok(req) = surf::get(&self.url).query(&params) {
                    match req.await {
                        Ok(_) => {}
                        Err(e) => error!("Error posting to Neosegment {:?}", e),
                    }
                }
            }
        }
    }
}

use std::{collections::HashMap, time::Duration};

use crate::comms::{QueryNow, QueryResult};

use dyn_fmt::AsStrFormatExt;
use log::{error, info};
use serde::{Deserialize, Serialize};
use surf::Url;
use xactor::*;

#[derive(Serialize, Clone, Debug)]
struct PromQLParams {
    query: String, //Prometheus expression query string.
}

type DataTuple = (f64, String);

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
enum PromQlResultData {
    Scalar(DataTuple),
    Matrix {
        metric: HashMap<String, String>,
        value: Vec<Vec<DataTuple>>,
    },
    Vector {
        metric: HashMap<String, String>,
        value: Vec<DataTuple>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PromQlInnerResult {
    resultType: String,
    result: PromQlResultData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PromQLResult {
    status: String,
    data: Option<PromQlInnerResult>,
    errorType: Option<String>,
    error: Option<String>,
    warnings: Option<Vec<String>>,
}

impl PromQLResult {
    pub fn is_scalar(&self) -> bool {
        self.status == "success"
            && self.data.is_some()
            && self.data.as_ref().filter(|d| d.resultType == "scalar").is_some()
    }
}

pub(crate) struct PrometheusSource {
    url: Url,
    query: String,
    resolution: Duration,
}

impl PrometheusSource {
    pub fn new<I: Into<String>, U: Into<Url>>(url: U, query: I, resolution: Duration) -> Self {
        PrometheusSource {
            url: url.into(),
            query: query.into(),
            resolution,
        }
    }
}

#[async_trait::async_trait]
impl Actor for PrometheusSource {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        ctx.send_interval(QueryNow, self.resolution);
        info!("PrometheusSource set up");
        Ok(())
    }
}

#[async_trait::async_trait]
impl Handler<QueryNow> for PrometheusSource {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: QueryNow) {
        let params = PromQLParams {
            query: self.query.clone(),
        };
        match surf::get(&self.url)
            .query(&params)
            .expect("Couldn't create request")
            .recv_json::<PromQLResult>()
            .await
        {
            Ok(response) if response.is_scalar() => match &response.data.unwrap().result {
                PromQlResultData::Scalar((_ts, value)) => {
                    let mut addr = Broker::from_registry().await.expect("Actors are down :(");
                    let value = value
                        .parse()
                        .expect(&"Couldn't parse result:{}".format(&[value]));
                    let _ = addr.publish(QueryResult::Scalar {
                        value,
                        unit: "".to_string(),
                    });
                }
                _ => {}
            },
            Ok(response) => error!("Not a scalar result: {:?}", response),
            Err(e) => error!("Prometheus query error: {:?} ({})", e, self.query),
        }
    }
}

use xactor::*;

#[message]
#[derive(Clone, Debug)]
pub struct QueryNow;

#[message]
#[derive(Clone, Debug)]
pub enum QueryResult {
    Scalar { value: f64, unit: String },
}

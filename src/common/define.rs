use crate::common::request::BaseRequest;
use serde::{Deserialize, Serialize};

pub trait SD: Serialize + for<'de> Deserialize<'de> {}

impl<T> SD for T where T: Serialize + for<'de> Deserialize<'de> {}

pub type HttpFn<T: SD> = fn() -> (RequestFn, ResponseFn<T>);

pub type RequestFn = fn() -> BaseRequest;

pub type ResponseFn<T: SD> = fn(reqwest::Response) -> anyhow::Result<T>;

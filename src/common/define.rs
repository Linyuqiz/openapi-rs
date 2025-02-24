use crate::common::request::BaseRequest;
use crate::common::response::BaseResponse;
use serde::{Deserialize, Serialize};

pub type HttpFn<T: Serialize, U: Deserialize> = fn() -> (RequestFn<T>, ResponseFn<U>);

pub type RequestFn<T: Serialize> = fn() -> BaseRequest<T>;

pub type ResponseFn<U: Deserialize> = fn(reqwest::Response) -> anyhow::Result<BaseResponse<U>>;

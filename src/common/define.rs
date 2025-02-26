use crate::common::request::BaseRequest;
use std::pin::Pin;

pub type HttpFn<R> = fn() -> (RequestFn, AsyncResponseFn<R>);

pub type RequestFn = Box<dyn Fn() -> BaseRequest + Send + Sync>;

pub type AsyncResponseFn<T> = Box<
    dyn Fn(reqwest::Response) -> Pin<Box<dyn Future<Output = anyhow::Result<T>> + Send>>
        + Send
        + Sync,
>;

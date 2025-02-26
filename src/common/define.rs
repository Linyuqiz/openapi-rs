use std::pin::Pin;

pub type HttpFn<T, R> = fn() -> (RequestFn<T>, AsyncResponseFn<R>);

pub type RequestFn<T> = Box<dyn Fn() -> T + Send + Sync>;

pub type AsyncResponseFn<T> = Box<
    dyn Fn(reqwest::Response) -> Pin<Box<dyn Future<Output = anyhow::Result<T>> + Send>>
        + Send
        + Sync,
>;

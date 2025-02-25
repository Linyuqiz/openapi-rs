#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use anyhow::Result;
        use reqwest::Method;
        use std::fmt::Debug;
        use std::future::Future;
        use std::marker::PhantomData;
        use std::pin::Pin;

        // 基础请求类型
        #[derive(Debug, Default)]
        struct BaseRequest<T> {
            method: Method,
            uri: String,
            // 通过 PhantomData 表示与泛型 T 相关，但不实际存储数据
            _phantom: PhantomData<T>,
        }

        // 示例请求和响应数据类型
        #[derive(Debug, Default)]
        struct ZoneListRequest;

        #[derive(Default, Debug)]
        struct ZoneListResponse;

        #[derive(Debug)]
        struct BaseResponse<T> {
            error_code: String,
            error_msg: String,
            request_id: String,
            data: Option<T>,
        }

        // 请求回调：同步构造请求对象
        type RequestFn<T> = Box<dyn Fn() -> T + Send + Sync>;

        // 响应回调：接受异步返回的 reqwest::Response，并返回 Future
        type AsyncResponseFn<T> = Box<
            dyn Fn(reqwest::Response) -> Pin<Box<dyn Future<Output = Result<T>> + Send>>
                + Send
                + Sync,
        >;

        // 构造请求的回调函数
        fn request_fn() -> RequestFn<BaseRequest<ZoneListRequest>> {
            Box::new(|| BaseRequest {
                method: Method::GET,
                uri: "/v1/jobs/zones".to_string(),
                _phantom: PhantomData,
                ..Default::default()
            })
        }

        // 构造响应解析的回调函数
        fn response_fn() -> AsyncResponseFn<BaseResponse<ZoneListResponse>> {
            Box::new(|_response: reqwest::Response| {
                // 这里可以使用 _response 读取响应数据，本示例直接构造一个示例返回值
                Box::pin(async move {
                    Ok(BaseResponse {
                        error_code: "0".to_string(),
                        error_msg: "".to_string(),
                        request_id: "".to_string(),
                        data: Some(ZoneListResponse::default()),
                    })
                })
            })
        }

        // 模拟 API Client
        struct ApiClient;

        impl ApiClient {
            // 异步 send 方法，利用异步客户端和异步回调解析响应
            async fn send<T, R>(req_fn: RequestFn<T>, resp_fn: AsyncResponseFn<R>) -> Result<R>
            where
                T: Debug + Send + 'static,
                R: Debug + Send + 'static,
            {
                // 生成请求对象
                let request = req_fn();
                println!("发起请求到 URI: {:#?}", request);

                // 使用异步 reqwest 客户端发送请求
                let client = reqwest::Client::new();
                // 示例中使用 httpbin.org 来模拟请求（实际可以使用 request.uri() 组合完整 URL）
                let response = client.get("http://httpbin.org/get").send().await?;

                // 调用异步响应解析回调，并 await 其结果
                let parsed_response = resp_fn(response).await?;
                Ok(parsed_response)
            }
        }

        #[tokio::main]
        async fn main() -> Result<()> {
            let req_cb = request_fn();
            let resp_cb = response_fn();

            let response = ApiClient::send(req_cb, resp_cb).await?;
            println!("解析后的响应: {:#?}", response);
            Ok(())
        }
    }
}

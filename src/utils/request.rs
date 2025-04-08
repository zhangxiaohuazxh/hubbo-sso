use anyhow::Result;
use reqwest::{Client, ClientBuilder, RequestBuilder};
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

#[allow(dead_code)]
#[derive(Debug)]
enum MediaType {
    /* json */
    ApplicationJson,
    /* 表单 */
    FormUrlEncoded,
    /* 表单&二进制数据 */
    FormMultiPart,
}

#[inline]
fn get_request_client() -> Client {
    ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .gzip(true)
        .build()
        .unwrap()
}

#[allow(dead_code)]
async fn do_get<T>(
    url: &str,
    query: Option<T>,
    headers: Option<HashMap<&str, &str>>,
) -> Result<String>
where
    T: Serialize,
{
    let client = get_request_client();
    let mut request = client.get(url);
    if let Some(query) = query {
        request = request.query(&query);
    }
    execute(request, headers).await
}

#[allow(dead_code)]
async fn do_post<T>(
    url: &str,
    media_type: MediaType,
    body: Option<&T>,
    headers: Option<HashMap<&str, &str>>,
) -> Result<String>
where
    T: Serialize + ?Sized,
{
    let client = get_request_client();
    if let Some(body) = body {
        match media_type {
            MediaType::ApplicationJson => {
                let request = client
                    .post(url)
                    .header("Content-Type", "application/json;charset=UTF-8")
                    .json(body);
                execute(request, headers).await
            }
            MediaType::FormUrlEncoded => {
                let request = client
                    .post(url)
                    .header(
                        "Content-Type",
                        "application/x-www-form-urlencoded;charset=UTF-8",
                    )
                    .form(body);
                // let body = body as &dyn Any;
                // if let Some(map) = body.downcast_ref::<HashMap<&str, &str>>() {}
                execute(request, headers).await
            }
            MediaType::FormMultiPart => {
                todo!("文件上传暂不予实现")
            }
        }
    } else {
        execute(client.post(url), headers).await
    }
}

#[allow(dead_code)]
async fn execute(
    mut request: RequestBuilder,
    headers: Option<HashMap<&str, &str>>,
) -> Result<String> {
    request = request.header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36")
		.header("accept-encoding", "gzip, deflate, br, zstd")
		.header("cache-control", "max-age=0");
    if let Some(headers) = headers {
        for (k, v) in headers.into_iter() {
            request = request.header(k, v);
        }
    }
    Ok(request.send().await?.text().await?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[actix_rt::test]
    async fn test_do_get() -> Result<()> {
        let res = do_get(
            "http://localhost:8080/test/query",
            Some([("name", "k"), ("value", "v")]),
            None,
        )
        .await?;
        println!("请求响应信息 {res}");
        Ok(())
    }

    #[actix_rt::test]
    async fn test_do_post() -> Result<()> {
        let mut headers = HashMap::new();
        headers.insert("Refer", "https://www.baidu.com");
        headers.insert("AuthToken", "asahuin2w1ijoi121");
        let _ = do_post(
            "http://localhost:8080/test/form",
            MediaType::FormUrlEncoded,
            Some(&[("name", "key"), ("value", "测试数据")]),
            Some(headers),
        )
        .await?;
        let mut body = HashMap::new();
        body.insert("name", "key");
        body.insert("value", "测试数据");
        let _ = do_post(
            "http://localhost:8080/test/json",
            MediaType::ApplicationJson,
            Some(&body),
            None,
        )
        .await?;
        Ok(())
    }

    #[actix_rt::test]
    async fn test_serialize_json() -> Result<()> {
        let json_string = serde_json::to_string(&[("name", "key")])?;
        println!("json string {json_string}");
        Ok(())
    }
}

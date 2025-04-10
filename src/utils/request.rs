use anyhow::Result;
use log::{error, info};
use reqwest::{Client, ClientBuilder, RequestBuilder};
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;
use std::{env, sync::Once};

static LOGGING_ENABLED_INIT: Once = Once::new();

#[allow(dead_code)]
#[derive(Debug)]
pub enum MediaType {
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
        .timeout(Duration::from_secs(5))
        .gzip(true)
        .build()
        .unwrap()
}

#[allow(dead_code)]
pub async fn do_get<T>(
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
pub async fn do_post<T>(
    url: &str,
    media_type: MediaType,
    body: Option<T>,
    headers: Option<HashMap<&str, &str>>,
) -> Result<String>
where
    T: Serialize + ?Sized + Into<reqwest::Body>,
{
    let client = get_request_client();
    if let Some(body) = body {
        match media_type {
            MediaType::ApplicationJson => {
                let request = client
                    .post(url)
                    .header("Content-Type", "application/json;charset=UTF-8")
                    .json(&body);
                execute(request, headers).await
            }
            MediaType::FormUrlEncoded => {
                let request = client
                    .post(url)
                    .header(
                        "Content-Type",
                        "application/x-www-form-urlencoded;charset=UTF-8",
                    )
                    .body(body);
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
    let request = request.send().await;
    match request {
        Err(err) => {
            if log_enabled() {
                let url = err.url().unwrap();
                if err.is_timeout() {
                    error!("连接超时 {:?}", url)
                } else if err.is_connect() {
                    error!("连接拒绝 {:?} ", url)
                } else if err.is_redirect() {
                    error!("重定向错误 {:?}", url)
                } else {
                    error!("未知错误 {:?}\r\n{}", url, err)
                }
            }
            panic!("请求失败 {:?}", err)
        }
        Ok(resp) => {
            let headers = resp.headers();
            if log_enabled() {
                info!("响应内容 {:?}", headers);
            }
            let result = resp.text().await?;
            if log_enabled() {
                info!("响应数据\r\n{:#?}", result);
            }
            return Ok(result);
        }
    }
}

#[allow(unsafe_code)]
fn log_enabled() -> bool {
    static mut LOGGING_ENABLED: bool = false;
    unsafe {
        LOGGING_ENABLED_INIT.call_once(|| {
            LOGGING_ENABLED = env::var("LOGGING_REQUEST").is_ok();
        });
        LOGGING_ENABLED
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use log::log;

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
        let res = do_post(
            "http://localhost:8080/test/form",
            MediaType::FormUrlEncoded,
            Some("name=key&value='test'"),
            Some(headers),
        )
        .await?;
        println!("res {}", res);
        // let mut body = HashMap::new();
        // body.insert("name", "key");
        // body.insert("value", "测试数据");
        // let _ = do_post(
        //     "http://localhost:8080/test/json",
        //     MediaType::ApplicationJson,
        //     Some(&body),
        //     None,
        // )
        // .await?;
        Ok(())
    }

    #[actix_rt::test]
    async fn test_serialize_json() -> Result<()> {
        let json_string = serde_json::to_string(&[("name", "key")])?;
        println!("json string {json_string}");
        Ok(())
    }

    #[actix_rt::test]
    async fn test_parse_env_arg() -> Result<()> {
        let env = std::env::var("LOGGING_REQUEST");
        if let Ok(env) = env {
            println!("读取到了环境变量 {env}")
        } else {
            println!("读取失败")
        }
        Ok(())
    }

    #[test]
    fn test_logger() -> Result<()> {
        info!("start up");
        Ok(())
    }
}

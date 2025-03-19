use crate::constants::web::{ERROR_CODE, ERROR_MSG, SUCCESS_CODE, SUCCESS_MSG};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseWrapper<'a, T: Serialize> {
    /// 状态码,200 成功 500 失败,其余见系统错误代码
    pub code: u16,
    /// 状态消息,可能包含错误原因
    pub msg: &'a str,
    /// 响应数据载荷,可选地
    pub data: Option<T>,
}

impl<T> ResponseWrapper<'_, T>
where
    T: Serialize,
{
    /// 基础构造函数
    pub fn new(code: u16, msg: &'static str, data: Option<T>) -> Self {
        Self { code, msg, data }
    }

    pub fn success() -> Self {
        Self {
            code: SUCCESS_CODE,
            msg: SUCCESS_MSG,
            data: None,
        }
    }

    /// ### 没有数据需要返回时,泛型不要随便写,给定一个i32即可,不然会造成空间的浪费
    /// #### @param code 响应状态码
    /// #### @param msg 响应消息
    pub fn success_without_data(code: u16, msg: &'static str) -> Self
    where
        T: Serialize,
    {
        Self {
            code,
            msg,
            data: None,
        }
    }

    /// ### 成功的响应信息
    /// #### @param data 响应数据
    pub fn success_with_data(data: T) -> Self {
        Self {
            code: SUCCESS_CODE,
            msg: SUCCESS_MSG,
            data: Some(data),
        }
    }

    pub fn failed() -> Self {
        Self {
            code: ERROR_CODE,
            msg: ERROR_MSG,
            data: None,
        }
    }

    /// ### 没有数据需要返回时,泛型不要随便写,给定一个i32即可,不然会造成空间的浪费
    /// #### @param code 错误状态码
    /// #### @param msg 错误原因
    pub fn failed_without_data(code: u16, msg: &'static str) -> Self
    where
        T: Serialize,
    {
        Self {
            code,
            msg,
            data: None,
        }
    }

    /// ### 失败的响应信息
    /// #### @param data 响应数据
    pub fn failed_with_error_data(data: T) -> Self {
        Self {
            code: ERROR_CODE,
            msg: ERROR_MSG,
            data: Some(data),
        }
    }
}

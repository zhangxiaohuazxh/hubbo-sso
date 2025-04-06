use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

#[allow(unused)]
#[derive(Debug)]
pub enum QueryParam<'a> {
    Boolean(bool),
    Str(&'a str),
    String(String),
    Integer(usize),
    Double(f64),
}

impl<'a> QueryParam<'a> {
    pub fn val(&self) -> Result<String> {
        Ok(format!("{}", self))
    }
}

impl<'a> Display for QueryParam<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryParam::Boolean(boolean) => write!(f, "{}", boolean),
            QueryParam::Str(str) => write!(f, "{}", str),
            QueryParam::String(string) => write!(f, "{}", string),
            QueryParam::Integer(integer) => write!(f, "{}", integer),
            QueryParam::Double(double) => write!(f, "{}", double),
        }
    }
}

// todo 将v做成标量类型
#[derive(Debug, Default)]
pub struct QueryStringBuilder<'a, K>
where
    K: AsRef<str> + Sized + Hash + Eq + Display + Ord,
{
    query: BTreeMap<K, QueryParam<'a>>,
}

impl<'a, K> QueryStringBuilder<'a, K>
where
    K: AsRef<str> + Sized + Hash + Eq + Display + Ord,
{
    pub fn new_with_initial_data(data: BTreeMap<K, QueryParam<'a>>) -> Self {
        Self { query: data }
    }

    pub fn new() -> Self {
        Self {
            query: BTreeMap::new(),
        }
    }
    pub fn append_query_param(&mut self, (key, value): (K, QueryParam<'a>)) {
        self.query.insert(key, value);
    }

    pub fn query_string(&self) -> Result<String> {
        let mut query_string = String::new();
        for (index, tup) in self.query.iter().enumerate() {
            if index == 0 {
                query_string.push_str(&format!("{}={}", tup.0, tup.1));
            } else {
                query_string.push_str(&format!("&{}={}", tup.0, tup.1));
            }
        }
        Ok(query_string)
    }

    pub fn url_encode_query_string(self) -> Result<String> {
        let mut serializer = url::form_urlencoded::Serializer::new(String::new());
        for (key, val) in self.query.into_iter() {
            serializer.append_pair(key.as_ref(), &val.val()?);
        }
        Ok(serializer.finish().to_string())
    }

    pub fn url_encode_query_string_with_base_url(self, base_url: &str) -> Result<String> {
        let args = self
            .url_encode_query_string()
            .with_context(|| "url编码失败")?;
        Ok(format!("{}?{}", base_url, args))
    }
}

#[cfg(test)]
mod test {
    use crate::utils::web::{QueryParam, QueryStringBuilder};
    use anyhow::Result;
    use std::collections::BTreeMap;

    #[test]
    fn test_query_string_builder() -> Result<()> {
        let mut map = BTreeMap::new();
        map.insert("b", QueryParam::Boolean(true));
        map.insert("a", QueryParam::Integer(1));
        let query = QueryStringBuilder::new_with_initial_data(map);
        println!("query {:#?}", query.query_string()?);
        Ok(())
    }
}

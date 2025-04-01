use std::collections::HashMap;
use std::hash::Hash;

// todo 将v做成标量类型
#[derive(Debug)]
pub struct QueryStringBuilder<K, V>
where
    K: AsRef<str> + Sized + Hash + Eq,
    V: Sized,
{
    query: HashMap<K, V>,
}

impl<K, V> QueryStringBuilder<K, V>
where
    K: AsRef<str> + Sized + Hash + Eq,
    V: Sized,
{
    pub fn new(data: HashMap<K, V>) -> Self {
        Self { query: data }
    }
}

mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_query_string_builder() -> anyhow::Result<()> {
        let mut map = HashMap::new();
        map.insert(String::from_str("a")?, 1);
        map.insert(String::from_str("b")?, 2);
        let query = QueryStringBuilder::new(map);
        println!("query {:#?}", query);
        Ok(())
    }
}

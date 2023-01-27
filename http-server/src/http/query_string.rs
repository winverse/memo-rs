use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

// a=1&b=2&c&d=&e===&d=7&d=abc;
// a is Single
// d is Multiple
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Mutiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split("&") {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find("=") {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            // 기존에 값이 없다면 Single, 기존에 값이 있다면 multiple
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => *existing = Value::Mutiple(vec![prev_val, val]),
                    Value::Mutiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}

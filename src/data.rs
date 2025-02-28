pub mod entities
{
    use std::fmt;

    use serde::{Deserialize, Serialize};
    use serde_aux::prelude::*;

    #[derive(Clone)]
    #[derive(Debug)]
    pub struct PretendoElement {
        pub domain: String,
        pub pretendos: Vec<Pretendo>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
    pub struct Pretendo {
        pub id: Option<i32>,
        pub path: String,
        pub return_object: String,
        #[serde(deserialize_with = "deserialize_string_from_number")]
        pub status_code:  String,
        pub name: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
    pub struct Webhook {
        pub url: String,
        pub payload: String,
        pub http_verb: HttpVerbs,
    }

    impl Pretendo {
        pub fn new() -> Self {
            Self {
                id: None,
                path: String::default(),
                return_object: String::default(),
                status_code: String::default(),
                name: String::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HttpVerbs {
        GET,
        POST,
    }
    impl fmt::Display for HttpVerbs {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                HttpVerbs::GET => write!(f, "GET"),
                HttpVerbs::POST => write!(f, "POST"),
            }
        }
    }
}
pub mod entities
{
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
}
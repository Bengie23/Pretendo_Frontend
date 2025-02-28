#[allow(unused)]

pub mod http {
    use reqwest::Error;

    use crate::{data::entities::HttpVerbs, logger};
 
    pub struct PretendoHttpClient;
    
    impl PretendoHttpClient {
        pub  async fn backend_alive() ->bool {
            let mut alive = false;
            let client = reqwest::Client::new();
            let url = "http://pretendo.local/pretendo/ping";
            let response = client
                .get(url)
                .header("Content-Type", "application/json")
                .send()
                .await;
            match response {
                Ok(result) => {
                    logger::debug::println!("Status Code: {}", result.status());
        
                    let response_body = result.text().await.unwrap();
        
                    logger::debug::println!("Response body: \n{}", response_body);

                    if response_body == "pong"
                    {
                        alive = true;
                    }
                },
                Err(_error) => {

                    logger::debug::println!("error calling pretendo/ping");
                }
            }
            return alive;
        }

        pub  async fn get_domains() ->Vec<String> {
            let client = reqwest::Client::new();
            let mut domains = Vec::new();
            let url = "http://pretendo.local/api/domains";
            let response = client
                .get(url)
                .header("Content-Type", "application/json")
                .send()
                .await;
            match response {
                Ok(result) => {
                    logger::debug::println!("Status Code: {}", result.status());
        
                    let response_body = result.text().await.unwrap();
        
                    logger::debug::println!("Response body: \n{}", response_body);

                    domains = serde_json::from_str(response_body.as_str()).unwrap();
                },
                Err(_error) => {

                }
            }
            return domains;
        }

        pub async fn get_pretendos(domain: &String) -> Option<String> {
            let mut pretendos = None;
            let url = format!("http://pretendo.local/api/domain/{}/pretendos", domain);

            let client = reqwest::Client::new();

            let response = client
                .get(url)
                .header("Content-Type", "application/json")
                .send()
                .await;
            match response {
                Ok(result) => {
                    logger::debug::println!("Status Code: {}", result.status());
        
                    let response_body = result.text().await.unwrap();
        
                    logger::debug::println!("Response body: \n{}", response_body);

                    pretendos = Some(response_body);

                },
                Err(_error) => {

                }
            }
            return pretendos;
        }
        
        pub async fn get_webhooks(pretendo_id: &i32) -> Option<String> {
            let mut webhooks = None;
            let url = format!("http://pretendo.local/api/pretendo/{}/webhooks", pretendo_id);

            let client = reqwest::Client::new();

            let response = client
                .get(url)
                .header("Content-Type", "application/json")
                .send()
                .await;
            match response {
                Ok(result) => {
                    logger::debug::println!("Status Code: {}", result.status());
        
                    let response_body = result.text().await.unwrap();
        
                    logger::debug::println!("Response body: \n{}", response_body);

                    if response_body != "[]"{
                        webhooks = Some(response_body);
                    }

                },
                Err(_error) => {

                }
            }
            return webhooks;
        }
        
        pub async fn add_pretendo(domain: &String, path: &String, return_object: &String, name: &String, status_code: &String) ->Result<bool,Error> {
            let url = format!("http://pretendo.local/api/domain/{}/pretendos", domain);
            let return_object_json = format!(r#"{}"#, return_object);
            let json_data = format!(r##"{{ "path":"{}","returnObject":"{}", "name":"{}", "statusCode":"{}"}}"##, path, return_object_json , name, status_code);
            logger::debug::println!("{}", json_data);
            
            let client = reqwest::Client::new();

            let response = client
                .post(url)
                .header("Content-Type", "application/json; charset=utf-8")
                .body(json_data.to_owned())
                .send()
                .await;
            match response {
                Ok(result) => {
                    logger::debug::println!("Status Code: {}", result.status());
        
                    let response_body = result.text().await.unwrap();
        
                    logger::debug::println!("Response body: \n{}", response_body);
                    return Ok(true);

                },
                Err(_error) => {
                    return Err(_error);
                }
            }
            
        }
        
        pub async fn add_webhook(pretendo_id: &i32, webhook_url: &String, payload: &String, verb: &HttpVerbs) ->Result<bool,Error>{
            let url = format!("http://pretendo.local/api/pretendo/{}/webhooks", pretendo_id);
            let payload_json = format!(r#"{}"#, payload);
            let json_data = format!(r##"{{ "url":"{}","payload":"{}", "httpVerb":"{}"}}"##, webhook_url, payload_json, verb);
            logger::debug::println!("{}", json_data);
            
            let client = reqwest::Client::new();

            let response = client
                .post(url)
                .header("Content-Type", "application/json")
                .body(json_data.to_owned())
                .send()
                .await;
            match response {
                Ok(result) => {
                    logger::debug::println!("Status Code: {}", result.status());
        
                    let response_body = result.text().await.unwrap();
        
                    logger::debug::println!("Response body: \n{}", response_body);
                    return Ok(true);

                },
                Err(_error) => {
                    return Err(_error);
                }
            }
        }
    }

}
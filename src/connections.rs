pub mod http {
    use reqwest::Error;
 
    pub struct PretendoHttpClient;
    
    impl PretendoHttpClient {
        pub  async fn get_domains() ->Vec<String> {
            let client = reqwest::Client::new();
            let mut domains = Vec::new();
            let url = "http://pretendo.local/api/domain";
            let response = client
                .get(url)
                .header("Content-Type", "application/json")
                .send()
                .await;
            match response {
                Ok(result) => {
                    println!("Status Code: {}", result.status());
        
                    let response_body = result.text().await.unwrap();
        
                    println!("Response body: \n{}", response_body);

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
                    println!("Status Code: {}", result.status());
        
                    let response_body = result.text().await.unwrap();
        
                    println!("Response body: \n{}", response_body);

                    pretendos = Some(response_body);

                },
                Err(_error) => {

                }
            }
            return pretendos;
        }
        
        pub async fn add_pretendo(domain: &String, path: &String, return_object: &String, name: &String, status_code: &String) ->Result<bool,Error> {
            let url = format!("http://pretendo.local/api/domain/{}/pretendos", domain);
            let return_object_json = format!(r#"{}"#, return_object);
            let json_data = format!(r##"{{ "path":"{}","returnObject":"{}", "name":"{}", "statusCode":"{}"}}"##, path, return_object_json , name, status_code);
            println!("{}", json_data);
            
            let client = reqwest::Client::new();

            let response = client
                .post(url)
                .header("Content-Type", "application/json")
                .body(json_data.to_owned())
                .send()
                .await;
            match response {
                Ok(result) => {
                    println!("Status Code: {}", result.status());
        
                    let response_body = result.text().await.unwrap();
        
                    println!("Response body: \n{}", response_body);
                    return Ok(true);

                },
                Err(_error) => {
                    return Err(_error);
                }
            }
            
        }
    }

}
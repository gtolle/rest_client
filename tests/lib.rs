#[cfg(test)]
mod test {
    extern crate rest_client;
    extern crate rustc_serialize;
    use self::rest_client::RestClient;
    use self::rustc_serialize::json::Json;

    #[test]
    fn test_get() {
        let response = RestClient::get("http://www.reddit.com/hot.json?limit=1").unwrap();
        let response_json = Json::from_str(&response.body).unwrap();
        assert!(response_json.as_object().unwrap().get(&"data".to_string()).unwrap().as_object().unwrap().get(&"children".to_string()).unwrap().as_array().unwrap().len() == 1);
    }
    
    #[test]
    fn test_get_with_params() {
        let response = RestClient::get_with_params("http://www.reddit.com/hot.json", &[("limit", "1")]).unwrap();
        let response_json = Json::from_str(&response.body).unwrap();
        assert!(response_json.as_object().unwrap().get(&"data".to_string()).unwrap().as_object().unwrap().get(&"children".to_string()).unwrap().as_array().unwrap().len() == 1);
    }
}

extern crate rest_client;
extern crate serialize;

use rest_client::RestClient;
use serialize::json;

#[deriving(Decodable, Encodable)]
pub struct TestStruct  {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}

fn main() {
    println!("{}", RestClient::get("http://www.reddit.com/hot.json?limit=1").unwrap());

    let response = RestClient::get("http://www.reddit.com/hot.json?limit=1").unwrap();
    println!("{:d} {} {} {}", response.code, response.status, response.headers, response);

    for header in response.headers.iter() {
        println!("{}", header);
    }

    let response_json = json::from_str(response.body.as_slice()).unwrap();
    println!("{}", response_json.find(&"data".to_string()).unwrap().find(&"children".to_string()));
    
    println!("{}", RestClient::get_with_params("http://www.reddit.com/hot.json", 
                                               [("limit", "1")]).unwrap());

    let object = TestStruct {
        data_int: 1,
        data_str: "toto".to_string(),
        data_vector: vec![2,3,4,5],
    };

    println!("{}", RestClient::post("http://www.reddit.com/api/login.json",
                                    json::encode(&object).as_slice(), "application/json").unwrap());

    println!("{}", RestClient::post_with_params("http://www.reddit.com/api/login.json", 
                                                [("api_type", "json"),
                                                 ("user", "myusername"),
                                                 ("passwd", "mypassword"),
                                                 ("rem", "True")]).unwrap());

    println!("{}", RestClient::delete("http://www.reddit.com/hot.json?limit=1").unwrap());
    println!("{}", RestClient::delete_with_params("http://www.reddit.com/hot.json",
                                                  [("limit", "1")]).unwrap());
}


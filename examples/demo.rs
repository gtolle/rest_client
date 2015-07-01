extern crate rest_client;
extern crate rustc_serialize;

use rest_client::RestClient;
use rustc_serialize::json;

fn main() {
    println!("{}", RestClient::get("http://example.com/resource").unwrap());
    
    println!("{}", RestClient::get_with_params("http://example.com/resource", 
                                               &[("id", "50"), ("foo", "bar")]).unwrap());
    
    let object = TestStruct {
        data_int: 1,
        data_str: "toto".to_string(),
        data_vector: vec![2,3,4,5],
    };

    println!("{}", RestClient::post("http://example.com/resource",
                                    &json::encode(&object).unwrap(), "application/json").unwrap());
  
    println!("{}", RestClient::post_with_params("http://example.com/resource",
                                                &[("param1", "one"), ("param2", "two")]).unwrap());
  
    println!("{}", RestClient::delete("http://example.com/resource").unwrap());
    
    let response = RestClient::get("http://example.com/resource").unwrap();
    
    println!("{}", response.code); // -> 404
    
    for header in response.headers.iter() {
        println!("{}", header); // -> (Cache-Control, max-age=604800) ...
    }
    
    println!("{}", response.to_string());				  
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TestStruct  {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}

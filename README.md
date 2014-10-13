Rust REST Client
================

A simple HTTP and REST client for Rust, inspired by Ruby's [rest-client](https://github.com/rest-client/rest-client).

Built on top of the Hyper HTTP library.

Usage
-----

```
extern crate rest_client;
extern crate serialize;

use rest_client::RestClient;
use serialize::json;

fn main() {
    println!("{}", RestClient::get("http://example.com/resource").unwrap());
    
    println!("{}", RestClient::get_with_params("http://example.com/resource", 
                                               [("id", "50"), ("foo", "bar")]).unwrap());
    
    let object = TestStruct {
        data_int: 1,
        data_str: "toto".to_string(),
        data_vector: vec![2,3,4,5],
    };

    println!("{}", RestClient::post("http://example.com/resource",
                                    json::encode(&object).as_slice(), 
                                    "application/json").unwrap());
  
    println!("{}", RestClient::post_with_params("http://example.com/resource",
                                                [("param1", "one"), 
                                                 ("param2", "two")]).unwrap());
  
    println!("{}", RestClient::delete("http://example.com/resource").unwrap());
    
    let response = RestClient::get("http://example.com/resource").unwrap();
    
    println!("{:d}", response.code); // -> 404
    
    for header in response.headers.iter() {
        println!("{}", header); // -> (Cache-Control, max-age=604800) ...
    }
    
    println!("{}", response.to_string());				  
}

#[deriving(Decodable, Encodable)]
pub struct TestStruct  {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}
```

TODO
====

* Examine what parts of Hyper should get re-exposed
* Add support for cookies
* Multipart POST
* Unit tests
* Testing in real applications
* Refactoring

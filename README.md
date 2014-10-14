Rust REST Client
================

A simple HTTP and REST client for Rust, inspired by Ruby's [rest-client](https://github.com/rest-client/rest-client).

The goal: make common REST requests with the fewest lines of code.

Built on top of the Hyper HTTP library.

[Full API Documentation](https://gtolle.github.com/rest_client)

Usage
-----

```
extern crate rest_client;
extern crate serialize;

// One simple 'use' to get all the functionality, plus the 'extern crate'.
use rest_client::RestClient;

use serialize::json;

fn main() {
    
    // A simple GET is just a GET. You can print the response struct (it supports Show).
    
    println!("{}", RestClient::get("http://example.com/resource").unwrap());
    
    // You can use an array of tuples to create a GET with query parameters.
    
    println!("{}", RestClient::get_with_params("http://example.com/resource", 
                                               [("id", "50"), ("foo", "bar")]).unwrap());

    // You can also use an array of tuples to create a POST with form parameters. 
    // The client sets the content-type to application/x-www-form-urlencoded for you.
    
    println!("{}", RestClient::post_with_params("http://example.com/resource",
                                                [("param1", "one"), 
                                                 ("param2", "two")]).unwrap());

    // You can POST a string or a JSON object with just a string and a MIME type.
    
    let object = TestStruct {
        data_int: 1,
        data_str: "toto".to_string(),
        data_vector: vec![2,3,4,5],
    };
    
    println!("{}", RestClient::post("http://example.com/resource",
                                    json::encode(&object).as_slice(), 
                                    "application/json").unwrap());

    // PUT and PATCH are supported as well, just like POST.
    
    // You can delete a resource with a simple DELETE. delete_with_params works too.
    
    println!("{}", RestClient::delete("http://example.com/resource").unwrap());
    
    /*
      The response struct has a few fields
      code (a simple integer)
      body (a string)
      status (a typed response code, from Hyper)
      headers (typed headers from Hyper)
    */
    
    let response = RestClient::get("http://example.com/resource").unwrap();
    
    println!("{:d}", response.code); // -> 404
    
    for header in response.headers.iter() {
        println!("{}", header); // -> (Cache-Control, max-age=604800) ...
    }
    
    println!("{}", response.to_string());				  
    
    /*
      All of the underlying errors are passed up through 
      the RestError struct in the Result.
      
      pub enum RestError {
        UrlParseError(ParseError),
        HttpRequestError(HttpError),
        HttpIoError(IoError)
      }
    */
}

#[deriving(Decodable, Encodable)]
pub struct TestStruct  {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}
```

Examples
--------

```
    println!("{}", RestClient::get("http://www.reddit.com/hot.json?limit=1").unwrap());

    let response = RestClient::get("http://www.reddit.com/hot.json?limit=1").unwrap();
    
    let response_json = json::from_str(response.body.as_slice()).unwrap();

    println!("{}", response_json.find(&"data".to_string()).unwrap()
                                .find(&"children".to_string()).unwrap());
    
    println!("{}", RestClient::post_with_params("http://www.reddit.com/api/login.json", 
                                                [("api_type", "json"),
                                                 ("user", "myusername"),
                                                 ("passwd", "mypassword"),
                                                 ("rem", "True")]).unwrap());

```

TODO
----

* Examine what parts of Hyper should get re-exposed
* Add support for cookies
* Extend hyper to support basic auth (lots of APIs need it)
* Multipart POST
* Unit tests
* Testing in real applications
* Refactoring

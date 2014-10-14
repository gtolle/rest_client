#![experimental]

extern crate hyper;
extern crate url;

use hyper::Url;
use hyper::client::Request;
use hyper::method::{Get, Post, Delete, Put, Patch};

use std::fmt::{mod, Show};
use std::io::IoError;
use url::ParseError;
use hyper::HttpError;
use hyper::header::common::ContentLength;
use hyper::header::common::ContentType;

pub struct RestClient;

impl RestClient {
    // TODO: add cookies

    pub fn get(url_str:&str) -> Result<Response, RestError> {
        RestClient::new(Get, url_str, None, None, None)
    }

    pub fn get_with_params(url_str:&str, params:&[(&str, &str)]) -> Result<Response, RestError> {
        RestClient::new(Get, url_str, Some(params), None, None)
    }    
 
    pub fn post_with_params(url_str:&str, params:&[(&str, &str)]) -> Result<Response, RestError> {
        RestClient::pstar_with_params( Post, url_str, params )
    }

    pub fn post(url_str:&str, body:&str, content_type:&str) -> Result<Response, RestError> {
        RestClient::pstar( Post, url_str, body, content_type )
    }

    pub fn patch_with_params(url_str:&str, params:&[(&str, &str)]) -> Result<Response, RestError> {
        RestClient::pstar_with_params( Patch, url_str, params )
    }

    pub fn patch(url_str:&str, body:&str, content_type:&str) -> Result<Response, RestError> {
        RestClient::pstar( Patch, url_str, body, content_type )
    }
    
    pub fn put_with_params(url_str:&str, params:&[(&str, &str)]) -> Result<Response, RestError> {
        RestClient::pstar_with_params( Put, url_str, params )
    }

    pub fn put(url_str:&str, body:&str, content_type:&str) -> Result<Response, RestError> {
        RestClient::pstar( Put, url_str, body, content_type )
    }

    pub fn delete(url_str:&str) -> Result<Response, RestError> {
        RestClient::new(Delete, url_str, None, None, None)
    }

    pub fn delete_with_params(url_str:&str, params:&[(&str, &str)]) -> Result<Response, RestError> {
        RestClient::new(Delete, url_str, Some(params), None, None)
    }    

    fn pstar_with_params( method:hyper::method::Method, url_str:&str, params:&[(&str, &str)]) -> Result<Response, RestError> {
        let mut params_vec = Vec::new();
        for param in params.iter() {
            params_vec.push(*param);
        }

        let post_body = url::form_urlencoded::serialize(params_vec.into_iter(), None);

        RestClient::pstar( method, url_str, post_body.as_slice(), "application/x-www-form-urlencoded" )
    }

    fn pstar(method:hyper::method::Method, url_str:&str, body:&str, content_type:&str) -> Result<Response, RestError> {
        RestClient::new( method, url_str, None, Some(body), Some(content_type) )
    }

    pub fn new(method:hyper::method::Method, url_str:&str, url_params:Option<&[(&str, &str)]>, body:Option<&str>, content_type:Option<&str>) -> Result<Response, RestError> {
        let mut url = match Url::parse(url_str) {
            Ok(url) => url,
            Err(err) => return Err(UrlParseError(err))
        };

        match url_params {
            Some(params) => {
                let mut params_vec = Vec::new();
                for param in params.iter() {
                    params_vec.push(*param);
                }
                
                // TODO: write article talking about iter() vs into_iter()
                url.set_query_from_pairs(params_vec.into_iter());
            },
            None => ()
        };

        let mut req = match Request::new(method, url) {
            Ok(req) => req,
            Err(err) => return Err(HttpRequestError(err))
        };

        match body {
            Some(body) =>
                req.headers_mut().set(ContentLength(body.len())),
            None => 
                // needed so that hyper doesn't try to send Transfer-Encoding:
                // Chunked, which causes some servers (e.g. www.reddit.co) to
                // hang. is this a bug in the hyper client? why would it send
                // T-E: Ch as a header in a GET request?
                req.headers_mut().set(ContentLength(0))
        };

        match content_type {
            Some(content_type) =>
                req.headers_mut().set(ContentType(from_str(content_type).unwrap())),
            None => ()
        };

        let mut req_started = match req.start() {
            Ok(req) => req,
            Err(err) => return Err(HttpRequestError(err))
        };

        match body {
            Some(body) =>
                match req_started.write(body.as_bytes()) {
                    Ok(()) => (),
                    Err(err) => return Err(HttpIoError(err))
                },
            None => ()
        };

        let mut resp = match req_started.send() {
            Ok(resp) => resp,
            Err(err) => return Err(HttpRequestError(err))
        };

        let body = match resp.read_to_string() {
            Ok(body) => body,
            Err(err) => return Err(HttpIoError(err))
        };

        let rest_response = Response {
            code: resp.status as i32,
            status: resp.status,
            headers: resp.headers,
            body: body,
        };

        return Ok(rest_response);
    }
}

pub struct Response {
    pub code: i32,
    pub status: hyper::status::StatusCode,
    pub headers: hyper::header::Headers,
    pub body: String,
}

impl Show for Response {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.body.fmt(fmt)
    }
}

#[deriving(Show, PartialEq, Clone)]
pub enum RestError {
    UrlParseError(ParseError),
    HttpRequestError(HttpError),
    HttpIoError(IoError)
}

#[test]
fn it_works() {
}

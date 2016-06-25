#![deny(warnings)]
extern crate hyper;
extern crate getopts;
extern crate env_logger;

use std::env;

use hyper::Client;
use std::borrow::Cow;
mod parser;
    
use parser::Parser;
use parser::Method;
use parser::CustomHeader;
use std::io::Read;
use hyper::client::Response;
use hyper::client::RequestBuilder;
use hyper::header::{Headers};

fn main() {
    env_logger::init().unwrap();

    let client = match env::var("HTTP_PROXY") {
        Ok(mut proxy) => {
            // parse the proxy, message if it doesn't make sense
            let mut port = 80;
            if let Some(colon) = proxy.rfind(':') {
                port = proxy[colon + 1..].parse().unwrap_or_else(|e| {
                    panic!("HTTP_PROXY is malformed: {:?}, port parse error: {}", proxy, e);
                });
                proxy.truncate(colon);
            }
            Client::with_http_proxy(proxy, port)
        },
        _ => Client::new()
    };

    match Parser::parse_comandline(env::args().collect()) {

    	Method::Get{address: x, custom_headers: h} => {
            println!("GET Request on addres {}", x);
            let mut req = client.get(&x);
            req = add_headers(req, h);
            print_result(req.send().unwrap());
        }

    	Method::Head{address: x, custom_headers: h} => {
            println!("HEAD Request on address: {}", x);
            let mut req = client.head(&x);
            req = add_headers(req, h);
            print_result(req.send().unwrap());
        }

    	Method::Put{address: x, data: d, custom_headers: h} => {
            println!("PUT Reuqest on addres: {}", x);
            let mut req = client.put(&x);
            req = add_headers(req, h);
            print_result(req.body(&d).send().unwrap());
        }

    	Method::Post{address: x, data: d, custom_headers: h} => {
            println!("POST Reuqest on addres: {}", x);
            let mut req = client.post(&x);
            req = add_headers(req, h);
            print_result(req.body(&d).send().unwrap());
        }
		Method::Delete{address: x, data: d, custom_headers: h} => {
            println!("DELETE: Request on address {}", x);
            let mut req = client.delete(&x);
            req = add_headers(req, h);
            print_result(req.body(&d).send().unwrap());
        }
    };
}
   
fn add_headers(req: RequestBuilder, custom_headers: Vec<CustomHeader>) -> RequestBuilder {
    let mut headers = Headers::new();
    for custom_header in custom_headers {
        headers.set_raw(Cow::from(custom_header.key), vec![custom_header.value.into_bytes()]);
    }
    return req.headers(headers);
}

fn print_result(mut res: Response) {
    let mut buf = String::new();
    match res.read_to_string(&mut buf) {
        Ok(_) => println!(""),
        Err(a) => println!("ERR {}", a),
    }

    println!("Response: {}", res.status);   
    println!("{}", buf);
}
    


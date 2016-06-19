#![deny(warnings)]
extern crate hyper;
extern crate getopts;
extern crate env_logger;

use getopts::Options;
use getopts::Matches;
use std::env;
// use std::io;

use hyper::Client;
use hyper::header::Connection;

#[allow(dead_code)]
enum Method {
	Get {address: String},
	Head {address: String},
	Post {data: String, address: String},
	Put {data: String, address: String},
	Delete {data: String, address: String}
}

fn main() {
    env_logger::init().unwrap();

    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return;
        }
    };

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

    let res = client.get(&*url)
        .header(Connection::close())
        .send().unwrap();

    println!("Response: {}", res.status);
    println!("Headers:\n{}", res.headers);
    match parse_comandline(env::args().collect()) {
    	Method::Get{address: x} => println!("Get: {}", x),
    	Method::Head{address: x} => println!("Head: {}", x),
    	Method::Put{address: x, ..} => println!("Put: {}", x),
    	Method::Post{address: x, ..} => println!("Post: {}", x),
		Method::Delete{address: x, ..} => println!("Delete: {}", x),
    };
    
    // io::copy(&mut res, &mut io::stdout()).unwrap();
}

fn parse_comandline(arguments: Vec<String>) -> Method {

	let mut opts = Options::new();    
    opts.optopt("a", "data", "set data to post/put/delete request", "DATA");

    opts.optflag("p", "post", "making post request");
    opts.optflag("u", "put", "making put request");
    opts.optflag("d", "delete", "making delete request");
    opts.optflag("h", "head", "making head request");

	let mut options: Vec<String> = vec!["p".to_string(),
	 "u".to_string(),
	 "d".to_string(),
	 "h".to_string()];

	let matches = match opts.parse(&arguments[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };	

    if matches.opt_present("h") {
    	options.retain(|x| x != "h");
		check_if_only_one_option(&matches, &options);	
		return Method::Head{address: get_address_from_input(&matches)};
    }

     if matches.opt_present("d") {
    	options.retain(|x| x != "d");
		check_if_only_one_option(&matches, &options);	
		return Method::Delete{address: get_address_from_input(&matches), data: get_data_from_input(&matches)};
    }

     if matches.opt_present("u") {
    	options.retain(|x| x != "u");
		check_if_only_one_option(&matches, &options);	
		return Method::Put{address: get_address_from_input(&matches), data: get_data_from_input(&matches)};
    }

     if matches.opt_present("p") {
    	options.retain(|x| x != "p");
		check_if_only_one_option(&matches, &options);	
		return Method::Post{address: get_address_from_input(&matches), data: get_data_from_input(&matches)};
    }
    check_if_only_one_option(&matches, &options);	
	Method::Get{address: get_address_from_input(&matches)}	
}

fn check_if_only_one_option(input: &Matches, left_options: &Vec<String>) {
	if input.opts_present(left_options) {
		panic!("There should be only one method provided");
	}
}

fn get_address_from_input(input: &Matches) -> String {
	if !input.free.is_empty() {
        input.free[0].clone()
    } else {
         panic!("There is no address provided to request");
    }
}

#[allow(dead_code)]
fn get_data_from_input(input: &Matches) -> String {
	if let Some(data) = input.opt_str("a") {
		data
	} else {
	    panic!("There is no data provided to request");
	}
}
#![deny(warnings)]
extern crate getopts;

use getopts::Options;
use getopts::Matches;


pub enum Method {
	Get {address: String, custom_headers: Vec<CustomHeader>},
	Head {address: String, custom_headers: Vec<CustomHeader>},
	Post {data: String, address: String, custom_headers: Vec<CustomHeader>},
	Put {data: String, address: String, custom_headers: Vec<CustomHeader>},
	Delete {data: String, address: String, custom_headers: Vec<CustomHeader>}
}

pub struct CustomHeader {
	pub	key: String,
	pub	value: String
}

pub struct Parser {

}

impl Parser {

	pub fn parse_comandline(arguments: Vec<String>) -> Method {

		let mut opts = Options::new();    
	    opts.optopt("a", "data", "set data to post/put/delete request", "DATA");

	    opts.optopt("e", "headers", "set header to request. Format: key1:val1;key2:val2;...", "HEADERS");

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
			Parser::check_if_only_one_option(&matches, &options);	
			return Method::Head{
				address: Parser::get_address_from_input(&matches),
				custom_headers: Parser::get_headers_from_input(&matches)
			};
	    }

	     if matches.opt_present("d") {
	    	options.retain(|x| x != "d");
			Parser::check_if_only_one_option(&matches, &options);	
			return Method::Delete{
				address: Parser::get_address_from_input(&matches), 
				data: Parser::get_data_from_input(&matches),
				custom_headers: Parser::get_headers_from_input(&matches)
			};
	    }

	     if matches.opt_present("u") {
	    	options.retain(|x| x != "u");
			Parser::check_if_only_one_option(&matches, &options);	
			return Method::Put{
				address: Parser::get_address_from_input(&matches), 
				data: Parser::get_data_from_input(&matches),
				custom_headers: Parser::get_headers_from_input(&matches)
			};
	    }

	     if matches.opt_present("p") {
	    	options.retain(|x| x != "p");
			Parser::check_if_only_one_option(&matches, &options);	
			return Method::Post{
				address: Parser::get_address_from_input(&matches), 
				data: Parser::get_data_from_input(&matches),
				custom_headers: Parser::get_headers_from_input(&matches)
			};
	    }
	    Parser::check_if_only_one_option(&matches, &options);	
		Method::Get{
			address: Parser::get_address_from_input(&matches),
			custom_headers: Parser::get_headers_from_input(&matches)
		}	
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

	fn get_headers_from_input(input: &Matches) -> Vec<CustomHeader> {
		let mut ret_structs: Vec<CustomHeader>  = Vec::new();
		if let Some(data) = input.opt_str("e") {
			let headers: Vec<&str> = data.split(";").collect();
			for header in headers {
				let entry: Vec<&str> = header.split(":").collect();
				ret_structs.push(CustomHeader {key: entry[0].to_string(), value: entry[1].to_string()});
			}
		}
		return ret_structs;
	}

	fn get_data_from_input(input: &Matches) -> String {
		if input.opt_present("a") {
			if let Some(data) = input.opt_str("a") {
				data
			} else {
		    	panic!("There is no data provided to request");
			}
		} else {
		    "".to_string()
		}
	}
}
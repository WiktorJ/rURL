pub enum Method<'a> {
	Get {address: &'a str},
	Head {data: &'a str, address: &'a str},
	Post {data: &'a str, address: &'a str},
	Put {data: &'a str, address: &'a str},
	Delete {data: &'a str, address: &'a str}
}
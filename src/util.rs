use std::fs::File;
use std::io::prelude::*;
use sharp_pencil::Response;

fn read_file<'a> (s: &'a str) -> Response
{
	let mut f = File::open(s).unwrap();
	let mut contents = String::new();
	f.read_to_string(&mut contents);
	Response::from(contents)
}

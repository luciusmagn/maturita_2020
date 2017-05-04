#![feature(closure_to_fn_coercion)]
extern crate markdown;
extern crate sharp_pencil;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use markdown::file_to_html;
use sharp_pencil::{Pencil, Request, Response, PencilResult};

macro_rules! md
{
	($lit:expr) => { |_: &mut Request| { markdown_page($lit) } };
}

fn markdown_page(name: &str) -> PencilResult
{
	let body = file_to_html(
		Path::new(
			format!("web/{}.md", name).as_str()
		)
	).unwrap();
	let mut f = File::open("template.html").unwrap();
	let mut wrapper = String::new();
	f.read_to_string(&mut wrapper).unwrap();
	let contents = wrapper.replace("[[[contents]]]", body.as_ref());
	Ok(Response::from(contents))
}

fn main()
{
	let mut app = Pencil::new("web");
	/*let index = |_: &mut Request| { markdown_page("index") };
	let miniref = |_
	*/
	let index = md!("index");


	app.get("/", "index", index);
	app.enable_static_file_handling();
	app.run("127.0.0.1:80");
}

extern crate sharp_pencil;

use std::fs::File;
use std::io::prelude::*;
use sharp_pencil::{Pencil, Request, Response, PencilResult};

fn index(_: &mut Request) -> PencilResult
{
	let mut f = File::open("web/index.html").unwrap();
	let mut contents = String::new();
	f.read_to_string(&mut contents);

	Ok(Response::from(contents))
}

fn main()
{
	let mut app = Pencil::new("web");
	app.get("/", "index", index);
	app.enable_static_file_handling();
	app.run("127.0.0.1:80");
}

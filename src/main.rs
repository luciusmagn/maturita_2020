#![feature(drop_types_in_const)]
extern crate markdown;
extern crate sharp_pencil;
extern crate sha_1;
#[macro_use]
extern crate lazy_static;
extern crate ansi_term;

use std::fs::{File, metadata};
use std::path::Path;
use std::io::prelude::*;
use std::time::SystemTime;
use std::process::exit;
use std::collections::HashMap;
use std::sync::Mutex;

use markdown::file_to_html;
use ansi_term::Colour::{Green, Blue};
use sharp_pencil::{Pencil, PencilResult, Request, Response};

macro_rules! md
{
	($lit:expr) => { |_: &mut Request| { markdown_page($lit) } };
}

static TEMPLATE:&'static str = include_str!("../template.html");
lazy_static! { static ref PAGE_CACHE_MUT: Mutex<HashMap<String, (String, SystemTime)>> = Mutex::new(HashMap::new()); }

fn markdown_page(name: &str) -> PencilResult
{
	let mut page_cache = PAGE_CACHE_MUT.lock().unwrap();
	let body = if page_cache.contains_key(&format!("web/{}.md", name))
	{
		let p = format!("web/{}.md", name);
		let metadata = match metadata(&p)  { Ok(m) => m, _ => { return Ok(Response::from("404")); } };
		let date = match metadata.modified() { Ok(d) => d, _ => { return Ok(Response::from("404")); } };

		let mut entry = match page_cache.get_mut(&p)
		{
			Some(e) => e,
			None => unreachable!()
		};
		if entry.1 == date
		{
			entry.0.clone()
		}
		else
		{
			let mut f = match File::open(&p) { Ok(f) => f, _ => { return Ok(Response::from("404")); } };
			match f.read_to_string(&mut entry.0)
			{
				Ok(_) => { entry.0.clone() },
				Err(_) => { return Ok(Response::from("404")); }
			}
		}
	}
	else
	{
		let p = format!("web/{}.md", name);
		let metadata = match metadata(&p)  { Ok(m) => m, _ => { return Ok(Response::from("404")); } };
		let date = match metadata.modified() { Ok(d) => d, _ => { return Ok(Response::from("404")); } };
		let s = match file_to_html(Path::new(format!("web/{}.md", name).as_str()))
		{
			Ok(s) => s,
			Err(_) => { return Ok(Response::from("404")); },
		};
		page_cache.insert(p, (s.clone(), date.clone()));
		s
	};
	let mut f = match File::open("template.html")
	{
		Ok(f) => f,
		Err(_) =>
		{
			eprintln!("error: couldn't open template.html");
			exit(-1);
		}
	};
	let mut wrapper = String::new();
	f.read_to_string(&mut wrapper).unwrap();
	let contents = TEMPLATE.replace("[[[contents]]]", body.as_ref());
	Ok(Response::from(contents))
}

fn main()
{
	let mut app = 	Pencil::new("web");
	let index 	= 	md!("index");
	let miniref = 	md!("miniref");

	app.get("/", "index", index);
	app.get("/miniref", "miniref", miniref);
	app.before_request(
		|request|
		{
			println!(" {} {} from {}",
				Green.bold().paint(request.method.to_string()),
				request.url,
				Blue.bold().paint(request.remote_addr.to_string())
			);
			None
		}
	);
	app.enable_static_file_handling();

	app.run("0.0.0.0:80");
}

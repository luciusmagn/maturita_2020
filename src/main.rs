#![feature(label_break_value)]

extern crate toml;
extern crate comrak;
extern crate ansi_term;
extern crate light_pencil;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

use std::io;
use std::mem;
use std::path::Path;
use std::time::SystemTime;
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};
use std::fs::{read_dir, DirEntry};

use ansi_term::Colour::{Green, Blue};
use light_pencil::{Pencil, Request, PencilResult};

mod types;
mod util;

use util::{markdown_page, rust, pebbles};

// site w/ sidebar
macro_rules! md
{
	($lit:expr) => { Box::new(move |_: &mut Request| { markdown_page($lit, TEMPLATE) }) };
}

// site w/out sidebar
macro_rules! raw_md
{
	($lit:expr) => { Box::new(move |_: &mut Request| { markdown_page($lit, RAW_TEMPLATE) }) };
}

static RAW_TEMPLATE:&'static str = include_str!("../raw_template.html");
static TEMPLATE:&'static str = include_str!("../template.html");
static PEBBLES:&'static str = include_str!("../pebbles.html");
static RUST:&'static str = include_str!("../rust.html");

lazy_static! {
	static ref PAGE_CACHE_MUT: Mutex<HashMap<String, (String, SystemTime)>> = Mutex::new(HashMap::new());
	static ref NAME_CACHE: RwLock<Vec<String>> = RwLock::new(vec![]);
}

fn main()
{
	let mut app  =  Pencil::new("web");

	app.get("/",         "index",    md!("index"));

	app.before_request(
		|request|
		{
			println!(" {} {} from {}",
				Green.bold().paint(request.method.to_string()),
				request.url,
				Blue.bold().paint(request.remote_addr.to_string())
			);

			'dir_loop: for (name, ext) in read_dir("web")
				.unwrap()
				.map(io::Result::unwrap)
				.filter(|x| x.
					file_type()
					.unwrap()
					.is_file())
				.map(|x| x.path())
				.map(|x| (x.file_stem().map(|s| s.to_owned()), x.extension().map(|s| s.to_owned())))
				.filter(|(s, e)| s.is_some() && e.is_some())
				.map(|(f, e)|
					(f.unwrap()
						.to_str()
						.map(|s| s.to_owned()),
					e.unwrap()
						.to_str()
						.map(|s| s.to_owned())))
				.map(|(f, e)| (f.unwrap(), e.unwrap()))
			{
				'read_scope: {
					let list = NAME_CACHE.read().unwrap();

					if list.contains(&name) {
						continue 'dir_loop;
					}
				}

				'write_scope: {
					let mut list = NAME_CACHE.write().unwrap();
					let name_clone = name.clone();

					match ext.as_ref() {
						"md"  => request.app.get("/".to_string() + &name, &name, md!(&name_clone)),
						"rmd" => request.app.get("/".to_string() + &name, &name, raw_md!(&name_clone)),
						_ => (),
					}

					list.push(name);
				}
			}
			None
		}
	);
	app.enable_static_file_handling();

	app.run("0.0.0.0:80");
}

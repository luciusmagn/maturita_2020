extern crate toml;
extern crate comrak;
extern crate ansi_term;
extern crate light_pencil;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

use std::sync::Mutex;
use std::time::SystemTime;
use std::collections::HashMap;

use ansi_term::Colour::{Green, Blue};
use light_pencil::{Pencil, Request};

mod types;
mod util;

use util::{markdown_page, rust, pebbles};

macro_rules! md
{
	($lit:expr) => { |_: &mut Request| { markdown_page($lit) } };
}

static TEMPLATE:&'static str = include_str!("../template.html");
static PEBBLES:&'static str = include_str!("../pebbles.html");
static RUST:&'static str = include_str!("../rust.html");
lazy_static! { static ref PAGE_CACHE_MUT: Mutex<HashMap<String, (String, SystemTime)>> = Mutex::new(HashMap::new()); }


fn main()
{
	let mut app  =  Pencil::new("web");
	app.get("/",         "index",    md!("index"));
	app.get("/style",    "style",    md!("style"));
	app.get("/proj",     "proj",     md!("proj"));
	app.get("/articles", "articles", md!("articles"));
		app.get("/my_langs",   "my_langs",   md!("my_langs"));
		app.get("/langs_give", "langs_give", md!("langs_give"));

	app.get("/rocks_suck", "rocks_suck", md!("rocks_suck"));
		app.get("/rocks", "rocks", md!("rocks"));
		app.get("/sucks", "sucks", md!("sucks"));

	app.get("/pebbles", "pebbles", pebbles);
	app.get("/rust",    "rust",    rust);

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

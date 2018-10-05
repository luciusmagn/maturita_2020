use std::io::Read;
use std::process::Command;
use std::fs::{File, metadata};

use light_pencil::{Response, PencilResult};
use comrak::{ComrakOptions, markdown_to_html};

use PAGE_CACHE;
use STYLUS_CACHE;
use RAW_TEMPLATE;

pub fn markdown_page(name: &str, template: &str) -> PencilResult
{
	let mut page_cache = PAGE_CACHE.lock().unwrap();
	let ext = if template == RAW_TEMPLATE { "rmd" } else { "md" };
	let body = if page_cache.contains_key(&format!("web/{}.{}", name, ext))
	{
		let p = format!("web/{}.{}", name, ext);
		let metadata = match metadata(&p) { Ok(m) => m, _ => { return Ok(Response::from("404")); } };
		let date = match metadata.modified() { Ok(d) => d, _ => { return Ok(Response::from("404")); } };

		let entry = match page_cache.get_mut(&p)
		{
			Some(e) => e,
			None => unreachable!(),
		};

		if entry.1 == date
			{entry.0.clone()}
		else
		{
			let mut f = match File::open(&p) { Ok(f) => f, _ => { return Ok(Response::from("404")); } };
			let mut tmp_str = String::new();
			match f.read_to_string(&mut tmp_str)
			{
				Ok(_) =>
				{
					entry.0 = markdown_to_html(&tmp_str, &ComrakOptions::default());
					entry.1 = date;
					entry.0.clone()
				},
				Err(_) => { return Ok(Response::from("404")); }
			}
		}
	}
	else
	{
		let p = format!("web/{}.{}", name, ext);
		let metadata = match metadata(&p)  { Ok(m) => m, _ => { return Ok(Response::from("404")); } };
		let date = match metadata.modified() { Ok(d) => d, _ => { return Ok(Response::from("404")); } };

		let mut f = match File::open(&p) { Ok(f) => f, _ => { return Ok(Response::from("404")); } };
		let mut tmp_str = String::new();

		if f.read_to_string(&mut tmp_str).is_err()
			{return Ok(Response::from("404"));}

		let s = markdown_to_html(&tmp_str, &ComrakOptions::default());
		page_cache.insert(p, (s.clone(), date.clone()));
		s
	};
	let contents = template.replace("<contents/>", body.as_ref());
	Ok(Response::from(contents))
}

pub fn stylus(name: &str) -> PencilResult {
	println!("test");
	let mut stylus_cache = STYLUS_CACHE.lock().unwrap();
	let p = format!("web/{}.styl", name);
	let body = if stylus_cache.contains_key(&format!("web/{}.styl", name))
	{
		let metadata = match metadata(&p) { Ok(m) => m, _ => { return Ok(Response::from("404")); } };
		let date = match metadata.modified() { Ok(d) => d, _ => { return Ok(Response::from("404")); } };

		let entry = match stylus_cache.get_mut(&p)
		{
			Some(e) => e,
			None => unreachable!(),
		};

		if entry.1 == date
			{entry.0.clone()}
		else
		{
			let mut f = match File::open(&p) { Ok(f) => f, _ => { return Ok(Response::from("404")); } };
			let mut tmp_str = String::new();
			match f.read_to_string(&mut tmp_str)
			{
				Ok(_) =>
				{
					entry.0 = dbg!(std::str::from_utf8(&Command::new("stylus")
							.arg(p).arg("-p").output()
							.expect("can't start stylus, is it installed?").stdout)
						.expect("couldn't convert stylus to CSS").to_string());
					entry.1 = date;
					entry.0.clone()
				},
				Err(_) => { return Ok(Response::from("404")); }
			}
		}
	}
	else
	{
		let metadata = match metadata(&p)  { Ok(m) => m, _ => { return Ok(Response::from("404")); } };
		let date = match metadata.modified() { Ok(d) => d, _ => { return Ok(Response::from("404")); } };

		let mut f = match File::open(&p) { Ok(f) => f, _ => { return Ok(Response::from("404")); } };
		let mut tmp_str = String::new();

		if f.read_to_string(&mut tmp_str).is_err()
			{return Ok(Response::from("404"));}

		let s = dbg!(std::str::from_utf8(&Command::new("stylus")
				.arg(&p).arg("-p").output()
				.expect("can't start stylus, is it installed?").stdout)
			.expect("couldn't convert stylus to CSS").to_string());
		stylus_cache.insert(p, (s.clone(), date.clone()));
		s
	};
	let mut res = Response::from(body);
	res.set_content_type("text/css");
	Ok(res)
}

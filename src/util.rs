use std::io::Read;
use std::fs::{File, metadata};

use light_pencil::{Response, PencilResult};
use comrak::{ComrakOptions, markdown_to_html};

use RAW_TEMPLATE;
use PAGE_CACHE_MUT;

pub fn markdown_page(name: &str, template: &str) -> PencilResult
{
	let mut page_cache = PAGE_CACHE_MUT.lock().unwrap();
	let ext = if template == RAW_TEMPLATE { "rmd" } else { "md" };
	let body = if page_cache.contains_key(&format!("web/{}.{}", name, ext))
	{
		let p = format!("web/{}.{}", name, ext);
		let metadata = match metadata(&p) { Ok(m) => m, _ => { return Ok(Response::from("404")); } };
		let date = match metadata.modified() { Ok(d) => d, _ => { return Ok(Response::from("404")); } };

		let entry = match page_cache.get_mut(&p)
		{
			Some(e) => e,
			None => unreachable!()
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

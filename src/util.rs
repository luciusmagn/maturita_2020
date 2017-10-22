use std::io::Read;
use std::fs::{File, metadata};

use comrak::{ComrakOptions, markdown_to_html};
use light_pencil::{Response, PencilResult, Request};

use RUST;
use PEBBLES;
use PAGE_CACHE_MUT;
use types::{Index, RIndex};

pub fn markdown_page(name: &str, template: &str) -> PencilResult
{
	let mut page_cache = PAGE_CACHE_MUT.lock().unwrap();
	let body = if page_cache.contains_key(&format!("web/{}.md", name))
	{
		let p = format!("web/{}.md", name);
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
		let p = format!("web/{}.md", name);
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
	let contents = template.replace("[[[contents]]]", body.as_ref());
	Ok(Response::from(contents))
}

pub fn pebbles(_: &mut Request) -> PencilResult
{
	let index = if let Ok(i) = Index::read() {i} else {return Ok(Response::from("couldn't read index"));};
	let mut content = String::new();

	for entry in index.entries
	{
		let current = format!("
			<tr>
  				<td>{}</td>
  				<td>{}</td>
  				<td>{}</td>
  				<td><a href=\"{3}\">{3}</a></td>
  			</tr>
		", entry.name, entry.versions[0], entry.author, if let Some(s) = entry.repository {s} else {"none".to_string()}
		);
		content.push_str(&current);
	}

	Ok(Response::from(PEBBLES.replace("[[[contents]]]", content.as_ref())))
}

pub fn rust(_: &mut Request) -> PencilResult
{
	let index = if let Ok(i) =
		RIndex::read() {i} else {return Ok(Response::from("couldn't read index"));};
	let mut content = String::new(); 
	for entry in index.users
	{
		content.push_str(&format!("
			<tr>
  				<td>{}</td>
  				<td>{}</td>
  				<td>{}</td>
  			</tr>
		", entry.username, entry.name, entry.points
		));
	}

	Ok(Response::from(RUST.replace("[[[contents]]]", content.as_ref())))
}

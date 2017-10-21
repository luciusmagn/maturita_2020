use std::fs::File;
use std::io::Read;

use toml;

// Pebbles
#[derive(Clone, Serialize, Deserialize)]
pub struct Index
{
	pub entries: Vec<Entry>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Entry
{
	pub name: String,
	pub versions: Vec<String>,
	pub author: String,
	pub repository: Option<String>,
}

impl Index
{
	pub fn read() -> Result<Index, toml::de::Error>
	{
		let mut me = match File::open("web/static/pebbles/data/index")
		{
			Ok(f) => f,
			Err(_) =>
			{
				println!("  error: failed to open index");
				return toml::from_str("file not found");
			}
		};
		let mut contents = String::new();
		if me.read_to_string(&mut contents).is_err()
			{println!("  error: failed to read index");}
		toml::from_str(contents.as_ref())
	}
}

// Rustgrade
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RIndex
{
	pub users: Vec<User>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User
{
	pub username: String,
	pub name: String,
	pub points: i32,
	pub log: Vec<LogEntry>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LogEntry
{
	pub amount: i32,
	pub reason: String,
	pub timestamp: String,
}

impl RIndex
{
	pub fn read() -> Result<RIndex, toml::de::Error>
	{
		let mut contents = String::new();
		let mut me = match File::open("web/static/rust/index")
		{
			Ok(f) => f,
			Err(e) =>
			{
				println!("{}", e);
				let _ = File::create("web/static/rust/index");
				return Ok(RIndex
				{
					users: Vec::new()
				});
			}
		};

		me.read_to_string(&mut contents).unwrap_or(0);
		toml::from_str(contents.as_ref())
	}
}

use std::process::Command;

fn main()
{
	assert!(Command::new("curl")
		.arg("-o")
		.arg("web/awesome_rust.md")
		.arg("https://raw.githubusercontent.com/rust-unofficial/awesome-rust/master/README.md")
		.output()
		.expect("couldn't launch curl. Do you have it installed?")
		.status
		.success())
}

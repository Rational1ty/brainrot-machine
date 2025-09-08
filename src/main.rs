use std::process::Command;

fn main() {
	const URL: &str = "https://www.youtube.com/shorts";
	let args = ["/c", "start", "chrome", URL];

	Command::new("cmd")
		.args(args)
		.spawn()
		.unwrap();
}

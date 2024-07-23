use std::io::BufRead;

fn main() -> Result<(), anscii::Error> {
	let args = std::env::args();

	if args.len() == 1 {
		println!("{}", from_input()?);
	}

	Ok(())
}

fn from_input() -> Result<String, anscii::Error> {
	let mut buffer = String::new();
	let mut aggregator = String::new();
	let mut handle = std::io::stdin().lock();

	while let Ok(read) = handle.read_line(&mut buffer) {
		if read == 0 {
			break;
		} else {
			aggregator.push_str(&buffer[..read]);
		}
		buffer.clear();
	}

	anscii::parse(&aggregator)
}

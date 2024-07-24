use std::io::BufRead;

fn main() -> Result<(), anscii::Error> {
	let args: Vec<String> = std::env::args().collect();

	match args.len() {
		1 => println!("{}", from_input()?),
        2 => println!("{}", anscii::parse(&args[1])?),
		3 => {
			match args[2].as_ref() {
				"-f" | "--file" => {
					println!("{}", anscii::parse(include_str!("../assets/sample.xml"))?);
				}
				_ => eprint!("too many arguments"),
			}
		}
		_ => eprint!("too many / few arguments"),
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

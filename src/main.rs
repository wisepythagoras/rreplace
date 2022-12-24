use std::{
	env,
	fs,
	io,
	process,
};
use regex::Regex;
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
	target: String,
	replacement: String,
	text: String,
}

fn print_usage() {
	eprintln!("{} - Change occurences of a string in a file to another",
	          "rreplace".bold());
	eprintln!("Usage: rreplace <target> <replacement> <INPUT> <OUTPUT>");
	eprintln!("Usage: cat INPUT | rreplace <target> <replacement> > OUTPUT");
}

fn parse_args() -> Arguments {
	let args: Vec<String> = env::args().skip(1).collect();
	let mut file_lines = String::new();

	if args.len() == 0 {
		print_usage();
		process::exit(1);
	}

	if args.len() != 3 {
		let lines = io::stdin().lines();
		for line in lines {
			let line_text = line.unwrap();
			file_lines += &(line_text + &"\n".to_owned());
		}
	} else {
		let filename = args[2].clone();

	    file_lines = match fs::read_to_string(&filename) {
	    	Ok(v) => v,
	    	Err(e) => {
	    		eprintln!("{}: Failed to read from file '{}': {}",
	    		          "Error".red().bold(), filename,
	    		          e.kind().to_string().red().bold());
	    		process::exit(1);
	    	}
	    };
	}

	if args.len() != 4 && file_lines.is_empty() {
		print_usage();
		eprintln!("{}: Wrong number of arguments: Expected 4, got {}.",
		          "Error".red().bold(), args.len());
		process::exit(1);
	}

	Arguments {
		target: args[0].clone(),
		replacement: args[1].clone(),
		text: file_lines,
	}
}

fn replace(target: &str, replacement: &str, text: &str)
	-> Result<String, regex::Error>
{
	let regex = Regex::new(target)?;
	Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    let args = parse_args();

    let replaced_data = match replace(&args.target, &args.replacement, &args.text) {
    	Ok(v) => v,
    	Err(e) => {
    		eprintln!("{}: Failed to perform replace: {:?}",
    		          "Error".red().bold(), e);
    		process::exit(1);
    	}
    };

    eprintln!("{}", replaced_data);
}

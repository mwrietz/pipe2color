use colored::Colorize;
use std::io;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
        process::exit(1);
    }

    let color_arg: &str = &args[1];
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(len) => if len == 0 {
                return;
            } else {
                print!("{}", input.color(color_arg));
            } 
            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        }
    }
}

fn usage() {
    let prog_path = env::current_dir().unwrap();
    println!("\n{} v{}", prog_path.file_name().unwrap().to_str().unwrap(), env!("CARGO_PKG_VERSION"));
    println!("Usage: <COMMAND> | {} <COLOR>\n", prog_path.file_name().unwrap().to_str().unwrap());
}

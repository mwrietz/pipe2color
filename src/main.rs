//use colored::Colorize;
use std::env;
use std::io;
use std::process;

mod gh_repo_status;
mod tui_gen;

use crate::tui_gen::print_color_bold;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
        gh_repo_status::check_version().expect("check_version error");
        process::exit(1);
    }

    let color_arg: &str = &args[1];
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    gh_repo_status::check_version().expect("check_version error");
                    return;
                } else {
                    //print!("{}", input.color(color_arg).bold());
                    print_color_bold(&input, color_arg);
                }
            }
            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        }
    }
}

fn usage() {
    println!(
        "\n{} v{}",
        tui_gen::get_prog_name(),
        env!("CARGO_PKG_VERSION")
    );
    println!("Usage: <COMMAND> | {} <COLOR>\n", tui_gen::get_prog_name());
}

use colored::Colorize;
use std::io;
use std::env;
use std::process;

mod tui_gen;
mod gh_repo_status;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
        gh_repo_status::check_version()
            .expect("check_version error");
        process::exit(1);
    }

    let color_arg: &str = &args[1];
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(len) => if len == 0 {
                gh_repo_status::check_version()
                    .expect("check_version error");
                return;
            } else {
                print!("{}", input.color(color_arg).bold());
            } 
            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        }
    }

}

fn usage() {
    //let prog_path = env::current_dir().unwrap();
    //println!("\n{} v{}", prog_path.file_name().unwrap().to_str().unwrap(), env!("CARGO_PKG_VERSION"));
    println!("\n{} v{}", tui_gen::get_prog_name(), env!("CARGO_PKG_VERSION"));
    //println!("Usage: <COMMAND> | {} <COLOR>\n", prog_path.file_name().unwrap().to_str().unwrap());
    println!("Usage: <COMMAND> | {} <COLOR>\n", tui_gen::get_prog_name());
}

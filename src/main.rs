use std::{env, process};

use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = minigrep::CmdArgs::build(&args).unwrap_or_else(|op| {
        eprintln!("Problem parsing arguments: {op}");
        process::exit(1);
    });
    if let Err(err) = run(args) {
        eprintln!("{}", err.to_string());
        process::exit(1);
    }
}

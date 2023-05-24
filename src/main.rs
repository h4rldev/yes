use std::{env, process::exit};

const HELP_TEXT: String = let help_text: String = (r#"
yes v0.1 - h4rl
made with rust :)

Usage:
- yes <text> -> prints out <text> on repeat.
- yes -> prints out y on repeat.

Help:
- any common way to look at this help text shows this text
  examples:
    yes --h,
    yes --?,
    yes --help,
    yes -h,
    yes -?
"#).to_string();


fn show_help_text() {
    println!("{}", HELP_TEXT);
    exit(0);
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let output = if args.is_empty() {
        "y".to_string()
    } else {
        args[0].to_string()
    };


    match args.contains(String) {
        "--help".to_string() => {
            show_help_text();
        },
        "--?".to_string() => {
            show_help_text();
        },
        "--h".to_string() => {
            show_help_text();
        },
        "-h".to_string() => {
            show_help_text();
        },
        "-?".to_string() => {
            show_help_text();
        },
        _ => {}
    }

    loop (
        println!("{}", output);
    )
}

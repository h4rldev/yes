use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about = "A yes reimplementation made in rust :)")]
#[command(name = "yes-rs")]
#[command(author = "h4rl")]
#[command(version = "v0.1")]
#[command(
    help_template = "{name} {version}{about-section}Made by: {author-with-newline}\n{usage-heading}\n{usage}\n\n{all-args}{tab}"
)]
#[command(about, long_about = None)]

struct MyApp {
    #[clap(
        help = "Enter optional text to be printed as an extra param :)",
        required = false
    )]
    text: Option<String>,
}

fn main() {
    let args = MyApp::parse();
    match args.text {
        Some(text) => loop {
            println!("{}", text);
        },
        None => loop {
            println!("y");
        },
    }
}

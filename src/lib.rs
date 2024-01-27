use std::error::Error;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}


type TasklistResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> TasklistResult<()> {
    Ok(())
}

pub fn run(config: ()) -> TasklistResult<()> {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
    Ok(config)
}


extern crate tokio;

use std::env;
use std::error::Error;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn Error>> {
    let name = env::args().nth(1).unwrap_or_else(|| "world".into());
    println!("Hello, {}!", name);

    // TODO: Replace with your application logic.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_ok() {
        assert!(run().is_ok());
    }
}
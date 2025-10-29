extern crate tokio;

mod server;
use server::ServerController;

mod config;
use config::ConfigController;

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
    ConfigController::read_config().await?;
    ServerController::start().await?;

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
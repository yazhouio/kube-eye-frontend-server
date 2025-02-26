use tokio::main;
use crate::error::Result;
mod server;
mod config;
mod error;
mod api;
mod typst_lib;

#[main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}

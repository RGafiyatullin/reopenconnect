// #![feature(backtrace)]
// #![feature(backtrace_frames)]

use std::error::Error as StdError;

use ::structopt::StructOpt;
use ::thiserror::private::AsDynError;

#[tokio::main]
async fn main() {
    let _ = ::dotenv::dotenv();
    if let Err(e) = ::reopenconnect::cli::ReOpenConnect::from_args().run().await {
        eprintln!("Failure:");
        print_err(e.as_dyn_error());
        std::process::exit(1)
    }
}

fn print_err(e: &dyn StdError) {
    eprintln!("- {}", e);
    if let Some(e) = e.source() {
        print_err(e)
    }
}

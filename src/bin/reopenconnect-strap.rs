use ::reopenconnect::cli::ReOpenConnectStrap;
use ::structopt::StructOpt;

fn main() {
    ReOpenConnectStrap::from_args().run().expect("Failure")
}

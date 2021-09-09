use structopt::StructOpt;

#[tokio::main]
async fn main() {
    ::reopenconnect::cli::Authenticate::from_args()
        .run()
        .await
        .expect("Failure")
}

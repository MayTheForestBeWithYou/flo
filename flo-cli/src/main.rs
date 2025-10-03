use crate::cli::run;

mod cli;

#[tokio::main]
async fn main() {
    let result = run().await;
    if let Err(err) = result {
        eprintln!("[flo error]: {err:?}");
        std::process::exit(1);
    }
}

#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    return run().await;
}


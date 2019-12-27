extern crate actix_web;
mod vendor;
use vendor::bolt;
mod state;
mod router;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    bolt::strike().await
}

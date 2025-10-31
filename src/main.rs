use crate::application::init_services::init_services;

mod domain;
mod infrastructure;
mod application;

#[tokio::main]
async fn main() {
    init_services().await;
}

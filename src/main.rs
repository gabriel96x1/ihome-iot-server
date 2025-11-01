use crate::application::init_services::init_services;

mod domain;
mod application;
pub mod network;
mod rag;
mod tts;
mod stt;
mod web_search;
mod llm;

#[tokio::main]
async fn main() {
    
    tokio::spawn(async {
        init_services().await;
    });

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}

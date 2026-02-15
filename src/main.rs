use dotenv::dotenv;
use std::env;
use log::{info, warn, error};

mod api;
mod trading;
mod web;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    // Initialize environment
    dotenv().ok();
    env_logger::init();
    
    info!("🤖 Survival Trading Bot iniciando...");
    
    // Load configuration
    let wallet_address = env::var("HL_WALLET_ADDRESS").expect("HL_WALLET_ADDRESS no encontrada en .env");
    let webhook_url = env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL no encontrada en .env");
    
    info!("Wallet configurada: {}", wallet_address);
    info!("Discord webhook configurado");
    
    // TODO: Implementar lógica principal
    
    info!("Bot iniciado y listo para tradear por su supervivencia 🚀");
}
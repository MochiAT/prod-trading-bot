use dotenv::dotenv;
use log::{info, warn};
mod api;
mod trading;

use trading::strategy::ScalpingStrategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    info!("Iniciando bot de trading en testnet");

    let strategy = ScalpingStrategy::new("BTC-USDC".to_string());
    
    info!("Ejecutando operación de prueba...");
    match strategy.execute_test_trade().await {
        Ok(_) => info!("Operación de prueba completada exitosamente"),
        Err(e) => warn!("Error en operación de prueba: {}", e),
    }

    Ok(())
}
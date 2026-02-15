use crate::api::client::HyperliquidClient;
use log::{info, warn};
use std::error::Error;

pub struct ScalpingStrategy {
    client: HyperliquidClient,
    trading_pair: String,
    position_size: f64,
    stop_loss_percentage: f64,
}

impl ScalpingStrategy {
    pub fn new(trading_pair: String) -> Self {
        Self {
            client: HyperliquidClient::new(),
            trading_pair,
            position_size: 10.0, // Tamaño inicial para pruebas (10 USDC)
            stop_loss_percentage: 0.015, // 1.5% stop loss
        }
    }

    pub async fn execute_test_trade(&self) -> Result<(), Box<dyn Error>> {
        info!("Iniciando operación de prueba en {}", self.trading_pair);

        // Verificar balance
        let balance = self.client.get_balance().await?;
        info!("Balance actual: {:?}", balance);

        // Colocar orden de compra
        let current_price = 40000.0; // TODO: Obtener precio real del mercado
        let buy_price = current_price * 0.999; // Ligero descuento para asegurar ejecución
        
        info!("Intentando comprar {} a {}", self.position_size, buy_price);
        let buy_order = self.client
            .place_order(&self.trading_pair, true, self.position_size, buy_price)
            .await?;
        
        info!("Orden de compra colocada: {:?}", buy_order);

        // TODO: Esperar confirmación de la orden y colocar orden de venta
        
        Ok(())
    }
}
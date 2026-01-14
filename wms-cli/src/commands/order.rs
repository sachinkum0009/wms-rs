use color_eyre::eyre::Result;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing::info;

#[derive(Clone, Debug)]
pub struct Order {
    pub order_id: String,
    pub item: String,
    pub quantity: u32,
}

// In-memory stub order store
pub static ORDER_STORE: Lazy<Mutex<Vec<Order>>> = Lazy::new(|| Mutex::new(vec![]));

/// Create a new order (stub implementation)
pub async fn create(item: String, quantity: u32) -> Result<()> {
    info!("📝 Creating new order...");
    info!("🚧 This is a placeholder implementation");

    // Validate inputs
    if item.trim().is_empty() {
        color_eyre::eyre::bail!("Item name cannot be empty");
    }

    if quantity == 0 {
        color_eyre::eyre::bail!("Quantity must be greater than 0");
    }

    let order_id = format!("ORD-{:06}", fastrand::u32(100000..999999));

    // Store in the in-memory store
    {
        let mut orders = ORDER_STORE.lock().unwrap();
        orders.push(Order {
            order_id: order_id.clone(),
            item: item.clone(),
            quantity,
        });
    }

    info!("📦 Order details:");
    info!("  • Item: {}", item);
    info!("  • Quantity: {}", quantity);
    info!("✅ Order created successfully!");
    info!("📋 Order ID: {}", order_id);
    info!("💡 Future implementation will store this order in the database");

    Ok(())
}

/// List all orders (stub)
pub async fn list() -> Result<()> {
    info!("📋 Listing all orders (session limited):");
    let orders = ORDER_STORE.lock().unwrap();
    if orders.is_empty() {
        info!("No orders have been created in this session.");
    } else {
        for order in orders.iter() {
            info!(
                "  • {} | {} (Qty: {})",
                order.order_id, order.item, order.quantity
            );
        }
    }
    Ok(())
}

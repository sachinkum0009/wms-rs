pub trait Manager {
    /// Bidding for the traffic to allow different robots/vehicles to pass
    fn bid(&self) -> Result<bool, serde_json::Error>;
}

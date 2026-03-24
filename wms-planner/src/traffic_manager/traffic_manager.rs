use serde::{Deserialize, Serialize};

use crate::traffic_manager::Manager;

#[derive(Serialize, Deserialize)]
pub enum VehicleType {
    AMR,
    AGV,
}

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    id: i32,
    vehicle_type: VehicleType,
}

#[derive(Serialize, Deserialize)]
pub struct TrafficManager {
    vehicles: Vec<Vehicle>,
}

impl Manager for TrafficManager {
    fn bid(&self) -> Result<bool, serde_json::Error> {
        for vehicle in &self.vehicles {
            let type_label = match vehicle.vehicle_type {
                VehicleType::AGV => "AGV",
                VehicleType::AMR => "AMR",
            };
            println!("vehicle id: {} is of type: {}", vehicle.id, type_label);
            let vehicle_json = serde_json::to_string(vehicle)?;
            println!("vehicle json data: {}", vehicle_json);
        }
        Ok(false)
    }
}

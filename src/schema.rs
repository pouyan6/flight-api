use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlightPlan {
    pub flight_plan_id: String,
    pub altitude: u16,
    pub airspeed: u16,
    pub aircraft_identification: String,
    pub aircraft_type: String,
    pub arrival_airport: String,
    pub departing_airport: String,
    pub flight_type: String,
    pub departure_time: String,
    pub estimated_arrival_time: String,
    pub route: String,
    pub remarks: String,
    pub fuel_hours: u8,
    pub fuel_minutes: u8,
    pub number_onboard: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub api_key: String
}

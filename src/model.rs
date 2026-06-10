use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Job {
    pub job_id: u32,
    pub name: String,
    pub duration_hours: u32,
    pub power_kw: f64,
    pub earliest_start: NaiveDateTime,
    pub deadline: NaiveDateTime,
    pub priority: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Signal {
    pub timestamp: NaiveDateTime,
    pub electricity_price_eur_per_kwh: f64,
    pub carbon_g_per_kwh: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScheduledJob {
    pub job_id: u32,
    pub name: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub energy_kwh: f64,
    pub cost_eur: f64,
    pub carbon_kg: f64,
}
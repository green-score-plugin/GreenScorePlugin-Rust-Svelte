#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonConstants;

impl CarbonConstants {
    pub const KWH_PER_GB_TRANSFER: f64 = 0.519;
    pub const KWH_PER_GB_DATACENTER: f64 = 0.065;
    pub const KWH_PER_REQUEST: f64 = 0.00015;
    pub const BYTES_TO_GB: f64 = 1.0 / (1024.0 * 1024.0 * 1024.0);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmissionsBreakdown {
    pub transfer: f64,
    pub datacenter: f64,
    pub requests: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyMetrics {
    pub transfer: f64,
    pub datacenter: f64,
    pub requests: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMetrics {
    pub gb_transferred: f64,
    pub gb_resources: f64,
    pub requests: u32,
    pub load_time_ms: u64,
    pub carbon_intensity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub energy: EnergyMetrics,
    pub data: DataMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmissionsMetrics {
    pub total_emissions: f64,
    pub breakdown: EmissionsBreakdown,
    pub metrics: Metrics,
}

pub fn calculate_carbon_emissions(
    bytes_transferred: u64,
    bytes_resources: u64,
    requests: u32,
    load_time_ms: u64,
    carbon_intensity: f64,
) -> EmissionsMetrics {
    let gb_transferred = bytes_transferred as f64 * CarbonConstants::BYTES_TO_GB;
    let gb_resources = bytes_resources as f64 * CarbonConstants::BYTES_TO_GB;

    let transfer_energy = gb_transferred * CarbonConstants::KWH_PER_GB_TRANSFER;
    let datacenter_energy = gb_resources * CarbonConstants::KWH_PER_GB_DATACENTER;
    let request_energy = requests as f64 * CarbonConstants::KWH_PER_REQUEST;

    let total_energy = transfer_energy + datacenter_energy + request_energy;

    let transfer_emissions = (transfer_energy * carbon_intensity * 100.0).round() / 100.0;
    let datacenter_emissions = (datacenter_energy * carbon_intensity * 100.0).round() / 100.0;
    let request_emissions = (request_energy * carbon_intensity * 100.0).round() / 100.0;

    let total_emissions = (total_energy * carbon_intensity * 100.0).round() / 100.0;

    EmissionsMetrics {
        total_emissions,
        breakdown: EmissionsBreakdown {
            transfer: transfer_emissions,
            datacenter: datacenter_emissions,
            requests: request_emissions,
        },
        metrics: Metrics {
            energy: EnergyMetrics {
                transfer: transfer_energy,
                datacenter: datacenter_energy,
                requests: request_energy,
                total: total_energy,
            },
            data: DataMetrics {
                gb_transferred,
                gb_resources,
                requests,
                load_time_ms,
                carbon_intensity,
            },
        },
    }
}

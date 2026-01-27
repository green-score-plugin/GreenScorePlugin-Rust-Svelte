use backend::controllers::calculation_method::{
    calculate_carbon_emissions, CarbonConstants, EmissionsBreakdown, EmissionsMetrics,
    EnergyMetrics, DataMetrics,
};

#[test]
fn test_carbon_constants() {
    // Vérifie que les constantes correspondent aux valeurs du JavaScript
    assert_eq!(CarbonConstants::KWH_PER_GB_TRANSFER, 0.519);
    assert_eq!(CarbonConstants::KWH_PER_GB_DATACENTER, 0.065);
    assert_eq!(CarbonConstants::KWH_PER_REQUEST, 0.00015);
    assert_eq!(CarbonConstants::BYTES_TO_GB, 1.0 / (1024.0 * 1024.0 * 1024.0));
}

#[test]
fn test_calculate_carbon_emissions_zero_values() {
    let result = calculate_carbon_emissions(0, 0, 0, 0, 442.0);

    assert_eq!(result.total_emissions, 0.0);
    assert_eq!(result.breakdown.transfer, 0.0);
    assert_eq!(result.breakdown.datacenter, 0.0);
    assert_eq!(result.breakdown.requests, 0.0);
    assert_eq!(result.metrics.energy.total, 0.0);
}

#[test]
fn test_calculate_carbon_emissions_basic_transfer() {
    // Test avec 1 GB transféré
    let one_gb = 1024 * 1024 * 1024;
    let result = calculate_carbon_emissions(one_gb, one_gb, 1, 1000, 442.0);

    // Énergie de transfert attendue : 1 GB * 0.519 kWh/GB = 0.519 kWh
    // Émissions attendues : 0.519 * 442 = 229.398 gCO2
    let expected_transfer_emissions = 0.519 * 442.0;

    assert!(result.breakdown.transfer > 0.0);
    assert!((result.breakdown.transfer - expected_transfer_emissions).abs() < 0.1);
}

#[test]
fn test_calculate_carbon_emissions_datacenter_energy() {
    // Test avec des ressources datacenter
    let one_gb = 1024 * 1024 * 1024;
    let result = calculate_carbon_emissions(0, one_gb, 0, 1000, 442.0);

    // Énergie datacenter attendue : 1 GB * 0.065 kWh/GB = 0.065 kWh
    // Émissions attendues : 0.065 * 442 = 28.73 gCO2
    let expected_datacenter_emissions = 0.065 * 442.0;

    assert!(result.breakdown.datacenter > 0.0);
    assert!((result.breakdown.datacenter - expected_datacenter_emissions).abs() < 0.1);
}

#[test]
fn test_calculate_carbon_emissions_requests_energy() {
    // Test avec 1000 requêtes
    let result = calculate_carbon_emissions(0, 0, 1000, 1000, 442.0);

    // Énergie requêtes attendue : 1000 * 0.00015 kWh = 0.15 kWh
    // Émissions attendues : 0.15 * 442 = 66.3 gCO2
    let expected_requests_emissions = 1000.0 * 0.00015 * 442.0;

    assert!(result.breakdown.requests > 0.0);
    assert!((result.breakdown.requests - expected_requests_emissions).abs() < 0.1);
}

#[test]
fn test_calculate_carbon_emissions_realistic_scenario() {
    // Scénario réaliste : 5 MB transférés, 10 MB ressources, 50 requêtes
    let bytes_transferred = 5 * 1024 * 1024;
    let bytes_resources = 10 * 1024 * 1024;
    let requests = 50;
    let carbon_intensity = 442.0;

    let result = calculate_carbon_emissions(
        bytes_transferred,
        bytes_resources,
        requests,
        5000,
        carbon_intensity
    );

    // Vérifications générales
    assert!(result.total_emissions > 0.0);
    assert!(result.breakdown.transfer > 0.0);
    assert!(result.breakdown.datacenter > 0.0);
    assert!(result.breakdown.requests > 0.0);

    // La somme des breakdowns doit être égale au total
    let sum = result.breakdown.transfer + result.breakdown.datacenter + result.breakdown.requests;
    assert!((result.total_emissions - sum).abs() < 0.1);

    // Les métriques d'énergie doivent être cohérentes
    assert!(result.metrics.energy.transfer > 0.0);
    assert!(result.metrics.energy.datacenter > 0.0);
    assert!(result.metrics.energy.requests > 0.0);
    assert!(result.metrics.energy.total > 0.0);
}

#[test]
fn test_calculate_carbon_emissions_different_carbon_intensities() {
    let bytes = 1024 * 1024 * 1024;

    // Test avec une intensité carbone faible (France, ~60 gCO2/kWh)
    let result_low = calculate_carbon_emissions(bytes, bytes, 10, 1000, 60.0);

    // Test avec une intensité carbone élevée (Pologne, ~700 gCO2/kWh)
    let result_high = calculate_carbon_emissions(bytes, bytes, 10, 1000, 700.0);

    // Les émissions doivent être proportionnelles à l'intensité carbone
    assert!(result_high.total_emissions > result_low.total_emissions);
    let ratio = result_high.total_emissions / result_low.total_emissions;
    assert!((ratio - (700.0 / 60.0)).abs() < 0.1);
}

#[test]
fn test_emissions_breakdown_structure() {
    let breakdown = EmissionsBreakdown {
        transfer: 100.0,
        datacenter: 50.0,
        requests: 25.0,
    };

    assert_eq!(breakdown.transfer, 100.0);
    assert_eq!(breakdown.datacenter, 50.0);
    assert_eq!(breakdown.requests, 25.0);
}

#[test]
fn test_energy_metrics_structure() {
    let energy = EnergyMetrics {
        transfer: 0.1,
        datacenter: 0.05,
        requests: 0.025,
        total: 0.175,
    };

    assert_eq!(energy.transfer, 0.1);
    assert_eq!(energy.datacenter, 0.05);
    assert_eq!(energy.requests, 0.025);
    assert_eq!(energy.total, 0.175);
}

#[test]
fn test_data_metrics_structure() {
    let data = DataMetrics {
        gb_transferred: 1.0,
        gb_resources: 2.0,
        requests: 100,
        load_time_ms: 5000,
        carbon_intensity: 442.0,
    };

    assert_eq!(data.gb_transferred, 1.0);
    assert_eq!(data.gb_resources, 2.0);
    assert_eq!(data.requests, 100);
    assert_eq!(data.load_time_ms, 5000);
    assert_eq!(data.carbon_intensity, 442.0);
}

#[test]
fn test_emissions_metrics_complete_structure() {
    let result = calculate_carbon_emissions(
        1024 * 1024 * 1024,
        2 * 1024 * 1024 * 1024,
        100,
        5000,
        442.0
    );

    // Vérifie que toutes les structures sont correctement remplies
    assert!(result.metrics.energy.transfer > 0.0);
    assert!(result.metrics.energy.datacenter > 0.0);
    assert!(result.metrics.energy.requests > 0.0);
    assert!(result.metrics.energy.total > 0.0);

    assert!(result.metrics.data.gb_transferred > 0.0);
    assert!(result.metrics.data.gb_resources > 0.0);
    assert_eq!(result.metrics.data.requests, 100);
    assert_eq!(result.metrics.data.load_time_ms, 5000);
    assert_eq!(result.metrics.data.carbon_intensity, 442.0);
}

#[test]
fn test_calculate_carbon_emissions_precision() {
    // Test de précision avec des valeurs décimales
    let result = calculate_carbon_emissions(
        1_500_000, // 1.5 MB
        3_000_000, // 3 MB
        15,
        2500,
        442.0
    );

    // Vérifier que les résultats sont arrondis à 2 décimales
    assert_eq!(result.total_emissions, (result.total_emissions * 100.0).round() / 100.0);
    assert_eq!(result.breakdown.transfer, (result.breakdown.transfer * 100.0).round() / 100.0);
    assert_eq!(result.breakdown.datacenter, (result.breakdown.datacenter * 100.0).round() / 100.0);
    assert_eq!(result.breakdown.requests, (result.breakdown.requests * 100.0).round() / 100.0);
}

#[test]
fn test_bytes_to_gb_conversion() {
    let one_gb = 1024 * 1024 * 1024;
    let result = calculate_carbon_emissions(one_gb, 0, 0, 1000, 442.0);

    // Vérifie que la conversion bytes vers GB est correcte
    assert!((result.metrics.data.gb_transferred - 1.0).abs() < 0.001);
}

#[test]
fn test_edge_case_large_values() {
    // Test avec de très grandes valeurs
    let large_bytes = u64::MAX / 2;
    let result = calculate_carbon_emissions(
        large_bytes,
        large_bytes,
        1000000,
        1000000,
        442.0
    );

    // Vérifie que le calcul ne provoque pas de dépassement
    assert!(result.total_emissions.is_finite());
    assert!(result.total_emissions > 0.0);
}

#[test]
fn test_comparison_with_javascript_example() {
    // Test avec les mêmes valeurs que dans le JavaScript
    // pour vérifier la cohérence entre les deux implémentations
    let bytes_transferred = 5_242_880; // 5 MB
    let bytes_resources = 10_485_760; // 10 MB
    let requests = 50;
    let carbon_intensity = 442.0;

    let result = calculate_carbon_emissions(
        bytes_transferred,
        bytes_resources,
        requests,
        5000,
        carbon_intensity
    );

    // Calcul manuel pour vérification
    let gb_transferred = bytes_transferred as f64 * CarbonConstants::BYTES_TO_GB;
    let gb_resources = bytes_resources as f64 * CarbonConstants::BYTES_TO_GB;

    let transfer_energy = gb_transferred * CarbonConstants::KWH_PER_GB_TRANSFER;
    let datacenter_energy = gb_resources * CarbonConstants::KWH_PER_GB_DATACENTER;
    let request_energy = requests as f64 * CarbonConstants::KWH_PER_REQUEST;

    let total_energy = transfer_energy + datacenter_energy + request_energy;
    let expected_emissions = total_energy * carbon_intensity;

    // Vérifie que le résultat correspond au calcul manuel (avec tolérance)
    assert!((result.total_emissions - expected_emissions).abs() < 0.1);
}

use std::collections::HashMap;

// Test economy calculations
#[test]
fn test_cost_calculation() {
    let base: f64 = 15.0;
    let cost_multiplier: f64 = 1.15;

    // First item costs base
    let cost_0: f64 = base * cost_multiplier.powi(0);
    assert!((cost_0 - 15.0).abs() < 0.001);

    // Second item costs base * 1.15
    let cost_1: f64 = base * cost_multiplier.powi(1);
    assert!((cost_1 - 17.25).abs() < 0.001);
}

#[test]
fn test_bulk_cost_formula() {
    let base: f64 = 15.0;
    let r: f64 = 1.15;

    // Geometric series sum: a * (r^n - 1) / (r - 1)
    // For buying 3 items starting from 0:
    let first_cost: f64 = base; // base * 1.15^0
    let bulk_cost: f64 = first_cost * (r.powi(3) - 1.0) / (r - 1.0);

    // Manual calculation: 15 + 17.25 + 19.8375 = 52.0875
    let manual: f64 = 15.0 + 15.0 * 1.15 + 15.0 * 1.15 * 1.15;

    assert!((bulk_cost - manual).abs() < 0.01);
}

#[test]
fn test_production_calculation() {
    // Simulate production with multipliers
    let base_rate: f64 = 0.1; // Solar panel
    let count: u64 = 10;
    let producer_multiplier: f64 = 2.0; // One upgrade
    let global_multiplier: f64 = 1.05; // Efficient wiring

    let production: f64 = base_rate * count as f64 * producer_multiplier * global_multiplier;

    // 0.1 * 10 * 2.0 * 1.05 = 2.1
    assert!((production - 2.1).abs() < 0.001);
}

#[test]
fn test_number_formatting() {
    // Test formatting function logic
    fn format_test(value: f64) -> String {
        if value < 1000.0 {
            if value < 10.0 {
                format!("{:.2}", value)
            } else if value < 100.0 {
                format!("{:.1}", value)
            } else {
                format!("{:.0}", value)
            }
        } else {
            let exponent = value.log10().floor() as i32;
            let mantissa = value / 10f64.powi(exponent);
            format!("{:.2}e{}", mantissa, exponent)
        }
    }

    assert_eq!(format_test(0.0), "0.00");
    assert_eq!(format_test(1.5), "1.50");
    assert_eq!(format_test(15.0), "15.0");
    assert_eq!(format_test(150.0), "150");
    assert_eq!(format_test(1500.0), "1.50e3");
    assert_eq!(format_test(1000000.0), "1.00e6");
}

#[test]
fn test_game_state_serialization() {
    // Test that HashMap serializes correctly
    let mut producers: HashMap<u32, u64> = HashMap::new();
    producers.insert(1, 10);
    producers.insert(2, 5);

    let json = serde_json::to_string(&producers).unwrap();
    let restored: HashMap<u32, u64> = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.get(&1), Some(&10));
    assert_eq!(restored.get(&2), Some(&5));
}

#[test]
fn test_offline_progress_cap() {
    // Verify offline progress is capped at 8 hours
    let max_offline_secs: u64 = 8 * 60 * 60;
    let elapsed: u64 = 24 * 60 * 60; // 24 hours

    let capped = elapsed.min(max_offline_secs);
    assert_eq!(capped, 8 * 60 * 60);
}

#[test]
fn test_max_affordable_binary_search() {
    // Test binary search for max affordable
    fn max_affordable(base: f64, owned: u64, energy: f64, max_qty: u64) -> u64 {
        let r: f64 = 1.15;

        let calc_cost = |qty: u64| -> f64 {
            if qty == 0 {
                return 0.0;
            }
            let first: f64 = base * r.powi(owned as i32);
            first * (r.powi(qty as i32) - 1.0) / (r - 1.0)
        };

        if calc_cost(1) > energy {
            return 0;
        }

        let mut low: u64 = 0;
        let mut high: u64 = max_qty.min(1000);

        while low < high {
            let mid = low + (high - low + 1) / 2;
            if calc_cost(mid) <= energy {
                low = mid;
            } else {
                high = mid - 1;
            }
        }

        low
    }

    // With exactly 15 energy, can afford 1 solar panel
    assert_eq!(max_affordable(15.0, 0, 15.0, 100), 1);

    // With 32.25 energy, can afford 2 (15 + 17.25)
    assert_eq!(max_affordable(15.0, 0, 32.25, 100), 2);

    // With 1000 energy, can afford many
    let affordable = max_affordable(15.0, 0, 1000.0, 100);
    assert!(affordable > 10);
}

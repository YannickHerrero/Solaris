/// Get cost multiplier - fixed at 1.15x for all producers
/// Consistent cost scaling across all producer types
pub fn get_cost_multiplier(_producer_id: u32) -> f64 {
    1.15
}

/// Calculate the cost of buying multiple items using geometric series
pub fn calculate_bulk_cost(base_cost: f64, owned: u64, quantity: u64, producer_id: u32) -> f64 {
    if quantity == 0 {
        return 0.0;
    }

    let r = get_cost_multiplier(producer_id);
    let first_cost = base_cost * r.powi(owned as i32);

    // Geometric series sum: a * (r^n - 1) / (r - 1)
    first_cost * (r.powi(quantity as i32) - 1.0) / (r - 1.0)
}

/// Calculate maximum number of items affordable
pub fn calculate_max_affordable(
    base_cost: f64,
    owned: u64,
    energy: f64,
    max_quantity: u64,
    producer_id: u32,
) -> u64 {
    if energy <= 0.0 || max_quantity == 0 {
        return 0;
    }

    // Binary search for maximum affordable quantity
    let mut low = 0u64;
    let mut high = max_quantity.min(10000); // Cap at reasonable maximum

    // First check if we can afford at least one
    if calculate_bulk_cost(base_cost, owned, 1, producer_id) > energy {
        return 0;
    }

    while low < high {
        let mid = low + (high - low + 1) / 2;
        let cost = calculate_bulk_cost(base_cost, owned, mid, producer_id);

        if cost <= energy {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_multiplier() {
        // All producers use 1.15x cost scaling
        assert!((get_cost_multiplier(1) - 1.15).abs() < 0.001);
        assert!((get_cost_multiplier(7) - 1.15).abs() < 0.001);
        assert!((get_cost_multiplier(8) - 1.15).abs() < 0.001);
        assert!((get_cost_multiplier(12) - 1.15).abs() < 0.001);
        assert!((get_cost_multiplier(13) - 1.15).abs() < 0.001);
        assert!((get_cost_multiplier(17) - 1.15).abs() < 0.001);
        assert!((get_cost_multiplier(18) - 1.15).abs() < 0.001);
        assert!((get_cost_multiplier(20) - 1.15).abs() < 0.001);
    }

    #[test]
    fn test_single_cost() {
        let base: f64 = 15.0;
        let producer_id = 1; // Uses 1.15x multiplier
                             // Buying 1 from 0 owned costs base
        assert!((calculate_bulk_cost(base, 0, 1, producer_id) - 15.0).abs() < 0.001);
        // Buying 1 when 1 owned costs base * 1.15
        assert!((calculate_bulk_cost(base, 1, 1, producer_id) - 17.25).abs() < 0.001);
    }

    #[test]
    fn test_bulk_cost() {
        let base: f64 = 15.0;
        let producer_id = 1; // Uses 1.15x multiplier
                             // Buying 1 should equal single cost
        assert!((calculate_bulk_cost(base, 0, 1, producer_id) - 15.0).abs() < 0.001);

        // Buying 2 from 0 should be sum of first two costs
        let expected: f64 = 15.0 + 15.0 * 1.15;
        assert!((calculate_bulk_cost(base, 0, 2, producer_id) - expected).abs() < 0.001);
    }

    #[test]
    fn test_max_affordable() {
        let base: f64 = 15.0;
        let producer_id = 1; // Uses 1.15x multiplier

        // With exactly 15 energy, can afford 1
        assert_eq!(calculate_max_affordable(base, 0, 15.0, 100, producer_id), 1);

        // With 32.25 energy, can afford 2 (15 + 17.25)
        assert_eq!(
            calculate_max_affordable(base, 0, 32.25, 100, producer_id),
            2
        );

        // With 0 energy, can afford 0
        assert_eq!(calculate_max_affordable(base, 0, 0.0, 100, producer_id), 0);
    }
}

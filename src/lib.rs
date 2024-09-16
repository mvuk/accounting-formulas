pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn break_even_point(fixed_costs: f64, selling_price: f64, variable_costs: f64) -> f64 {
    fixed_costs / (selling_price - variable_costs)
}

pub fn contribution_margin(selling_price: f64, variable_costs: f64) -> f64 {
    selling_price - variable_costs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_even_point() {
        let result = break_even_point(1000.0, 100.0, 50.0);
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_contribution_margin() {
        let result = contribution_margin(100.0, 50.0);
        assert_eq!(result, 50.0);
    }
}

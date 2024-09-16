pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn break_even_point(fixed_costs: f64, selling_price: f64, variable_costs: f64) -> f64 {
    fixed_costs / (selling_price - variable_costs)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

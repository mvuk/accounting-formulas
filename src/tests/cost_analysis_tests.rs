use accounting_formulas::{Cost, CostObject, CostAnalysis, CostType, CostClassification};
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

// Chapter 2 - Using Costs in Decision Making

    #[test]
    fn test_cost_creation_valid() {
        let cost = Cost::new(100.0, "variable", "direct").unwrap();
        assert_eq!(cost.amount, 100.0);
        assert_eq!(cost.cost_type, CostType::Variable);
        assert_eq!(cost.classification, CostClassification::Direct);
    }

    #[test]
    fn test_cost_creation_invalid_type() {
        let result = Cost::new(100.0, "invalid", "direct");
        assert!(result.is_err());
    }

    #[test]
    fn test_cost_creation_invalid_classification() {
        let result = Cost::new(100.0, "fixed", "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_cost_object_creation() {
        let co = CostObject::new("Product A".to_string());
        assert_eq!(co.name, "Product A");
        assert!(co.costs.is_empty());
    }

    #[test]
    fn test_cost_object_add_cost() {
        let mut co = CostObject::new("Product A".to_string());
        co.add_cost(Cost::new(100.0, "variable", "direct").unwrap());
        co.add_cost(Cost::new(200.0, "fixed", "indirect").unwrap());
        assert_eq!(co.costs.len(), 2);
    }

    #[test]
    fn test_cost_object_total_cost() {
        let mut co = CostObject::new("Product A".to_string());
        co.add_cost(Cost::new(100.0, "variable", "direct").unwrap());
        co.add_cost(Cost::new(200.0, "fixed", "indirect").unwrap());
        assert_eq!(co.total_cost(), 300.0);
    }

    #[test]
    fn test_cost_object_variable_fixed_breakdown() {
        let mut co = CostObject::new("Product A".to_string());
        co.add_cost(Cost::new(100.0, "variable", "direct").unwrap());
        co.add_cost(Cost::new(200.0, "fixed", "indirect").unwrap());
        co.add_cost(Cost::new(150.0, "variable", "indirect").unwrap());
        let (variable, fixed) = co.variable_fixed_breakdown();
        assert_eq!(variable, 250.0);
        assert_eq!(fixed, 200.0);
    }

    #[test]
    fn test_cost_object_direct_indirect_breakdown() {
        let mut co = CostObject::new("Product A".to_string());
        co.add_cost(Cost::new(100.0, "variable", "direct").unwrap());
        co.add_cost(Cost::new(200.0, "fixed", "indirect").unwrap());
        co.add_cost(Cost::new(150.0, "variable", "indirect").unwrap());
        let (direct, indirect) = co.direct_indirect_breakdown();
        assert_eq!(direct, 100.0);
        assert_eq!(indirect, 350.0);
    }

    #[test]
    fn test_cost_analysis_creation() {
        let ca = CostAnalysis::new();
        assert!(ca.cost_objects.is_empty());
    }

    #[test]
    fn test_cost_analysis_add_cost_object() {
        let mut ca = CostAnalysis::new();
        let co = CostObject::new("Product A".to_string());
        ca.add_cost_object(co);
        assert_eq!(ca.cost_objects.len(), 1);
        assert!(ca.cost_objects.contains_key("Product A"));
    }

    #[test]
    fn test_cost_analysis_total_costs() {
        let mut ca = CostAnalysis::new();

        let mut co1 = CostObject::new("Product A".to_string());
        co1.add_cost(Cost::new(100.0, "variable", "direct").unwrap());
        co1.add_cost(Cost::new(200.0, "fixed", "indirect").unwrap());

        let mut co2 = CostObject::new("Product B".to_string());
        co2.add_cost(Cost::new(150.0, "variable", "indirect").unwrap());
        co2.add_cost(Cost::new(250.0, "fixed", "direct").unwrap());

        ca.add_cost_object(co1);
        ca.add_cost_object(co2);

        assert_eq!(ca.total_costs(), 700.0);
    }

    #[test]
    fn test_cost_analysis_overall_variable_fixed_breakdown() {
        let mut ca = CostAnalysis::new();

        let mut co1 = CostObject::new("Product A".to_string());
        co1.add_cost(Cost::new(100.0, "variable", "direct").unwrap());
        co1.add_cost(Cost::new(200.0, "fixed", "indirect").unwrap());

        let mut co2 = CostObject::new("Product B".to_string());
        co2.add_cost(Cost::new(150.0, "variable", "indirect").unwrap());
        co2.add_cost(Cost::new(250.0, "fixed", "direct").unwrap());

        ca.add_cost_object(co1);
        ca.add_cost_object(co2);

        let (variable, fixed) = ca.overall_variable_fixed_breakdown();
        assert_eq!(variable, 250.0);
        assert_eq!(fixed, 450.0);
    }

    #[test]
    fn test_cost_analysis_overall_direct_indirect_breakdown() {
        let mut ca = CostAnalysis::new();

        let mut co1 = CostObject::new("Product A".to_string());
        co1.add_cost(Cost::new(100.0, "variable", "direct").unwrap());
        co1.add_cost(Cost::new(200.0, "fixed", "indirect").unwrap());

        let mut co2 = CostObject::new("Product B".to_string());
        co2.add_cost(Cost::new(150.0, "variable", "indirect").unwrap());
        co2.add_cost(Cost::new(250.0, "fixed", "direct").unwrap());

        ca.add_cost_object(co1);
        ca.add_cost_object(co2);

        let (direct, indirect) = ca.overall_direct_indirect_breakdown();
        assert_eq!(direct, 350.0);
        assert_eq!(indirect, 350.0);
    }
}
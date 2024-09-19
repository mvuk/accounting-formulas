// Managerial Accounting

// Chapter 2 - Using Costs in Decision-Making
// Building the cost object, fixed and variable costs.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum CostType {
    Fixed,
    Variable,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CostClassification {
    Direct,
    Indirect,
}

#[derive(Debug, Clone)]
pub struct Cost {
    pub amount: f64,
    pub cost_type: CostType,
    pub classification: CostClassification,
}

impl Cost {
    pub fn new(amount: f64, cost_type: &str, classification: &str) -> Result<Self, String> {
        let cost_type = match cost_type.to_lowercase().as_str() {
            "fixed" => CostType::Fixed,
            "variable" => CostType::Variable,
            _ => return Err("Invalid cost type. Use 'fixed' or 'variable'.".to_string()),
        };

        let classification = match classification.to_lowercase().as_str() {
            "direct" => CostClassification::Direct,
            "indirect" => CostClassification::Indirect,
            _ => return Err("Invalid classification. Use 'direct' or 'indirect'.".to_string()),
        };

        Ok(Cost {
            amount,
            cost_type,
            classification,
        })
    }
}

pub struct CostObject {
    pub name: String,
    pub costs: Vec<Cost>,
    pub units: u32,
    pub price_per_unit: f64,
}

impl CostObject {
    pub fn new(name: String, units: u32, price_per_unit: f64) -> Self {
        CostObject {
            name,
            costs: Vec::new(),
            units,
            price_per_unit,
        }
    }
    pub fn add_cost(&mut self, cost: Cost) {
        self.costs.push(cost);
    }

    pub fn total_cost(&self) -> f64 {
        self.costs.iter().map(|c| c.amount).sum()
    }

    pub fn variable_fixed_breakdown(&self) -> (f64, f64) {
        let variable = self.costs.iter()
            .filter(|c| c.cost_type == CostType::Variable)
            .map(|c| c.amount)
            .sum();
        let fixed = self.costs.iter()
            .filter(|c| c.cost_type == CostType::Fixed)
            .map(|c| c.amount)
            .sum();
        (variable, fixed)
    }

    pub fn direct_indirect_breakdown(&self) -> (f64, f64) {
        let direct = self.costs.iter()
            .filter(|c| c.classification == CostClassification::Direct)
            .map(|c| c.amount)
            .sum();
        let indirect = self.costs.iter()
            .filter(|c| c.classification == CostClassification::Indirect)
            .map(|c| c.amount)
            .sum();
        (direct, indirect)
    }

    pub fn contribution_margin_per_unit(&self) -> f64 {
        let (variable_costs, _) = self.variable_fixed_breakdown();
        self.price_per_unit - (variable_costs / self.units as f64)
    }

    pub fn contribution_margin_ratio(&self) -> f64 {
        self.contribution_margin_per_unit() / self.price_per_unit
    }

    pub fn break_even_units(&self) -> u32 {
        let (_, fixed_costs) = self.variable_fixed_breakdown();
        (fixed_costs / self.contribution_margin_per_unit()).ceil() as u32
    }
}

pub struct CostAnalysis {
    pub cost_objects: HashMap<String, CostObject>,
}

impl CostAnalysis {
    pub fn new() -> Self {
        CostAnalysis {
            cost_objects: HashMap::new(),
        }
    }

    pub fn add_cost_object(&mut self, cost_object: CostObject) {
        self.cost_objects.insert(cost_object.name.clone(), cost_object);
    }

    pub fn total_costs(&self) -> f64 {
        self.cost_objects.values().map(|co| co.total_cost()).sum()
    }

    pub fn overall_variable_fixed_breakdown(&self) -> (f64, f64) {
        let mut total_variable = 0.0;
        let mut total_fixed = 0.0;

        for co in self.cost_objects.values() {
            let (variable, fixed) = co.variable_fixed_breakdown();
            total_variable += variable;
            total_fixed += fixed;
        }

        (total_variable, total_fixed)
    }

    pub fn overall_direct_indirect_breakdown(&self) -> (f64, f64) {
        let mut total_direct = 0.0;
        let mut total_indirect = 0.0;

        for co in self.cost_objects.values() {
            let (direct, indirect) = co.direct_indirect_breakdown();
            total_direct += direct;
            total_indirect += indirect;
        }

        (total_direct, total_indirect)
    }
}


// #TODO implement above in appropriate chapter

pub fn break_even_point(fixed_costs: f64, selling_price: f64, variable_costs: f64) -> f64 {
    fixed_costs / (selling_price - variable_costs)
}

pub fn contribution_margin(selling_price: f64, variable_costs: f64) -> f64 {
    selling_price - variable_costs
}


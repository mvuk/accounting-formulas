use accounting_formulas::{CostObject, Cost};

fn main() -> Result<(), String>  {
    println!("Chapter 02");
    println!("\nProblem 2-26\n");

    println!("Breakeven analysis. Klear Camera Company is planning to introduce a new video camera. The camera’s selling price is projected to be $1,000 per unit. Variable manufacturing costs are estimated to be $500 per unit. Variable selling costs are 10% of sales dollars. The company expects the annual fixed manufacturing costs for the new camera to be $3.5 million.");
    println!("(a)Compute Klear’s contribution margin per unit and contribution margin ratio.");
    println!("(b)Determine the number of units Klear must sell to break even.\n");

    let mut klear_camera = CostObject::new("Klear Camera".to_string(), 1, 1000.0);

    // Add variable manufacturing costs
    klear_camera.add_cost(Cost::new(500.0, "variable", "direct")?);

    // Add variable selling costs (10% of sales dollars)
    klear_camera.add_cost(Cost::new(100.0, "variable", "indirect")?);

    // Add fixed manufacturing costs
    klear_camera.add_cost(Cost::new(3_500_000.0, "fixed", "indirect")?);

    // Calculate contribution margin per unit
    let cm_per_unit = klear_camera.contribution_margin_per_unit();
    println!("Contribution Margin per unit: ${:.2}", cm_per_unit);

    // Calculate contribution margin ratio
    let cm_ratio = klear_camera.contribution_margin_ratio();
    println!("Contribution Margin Ratio: {:.2}%", cm_ratio * 100.0);

    // Calculate break-even point
    let break_even = klear_camera.break_even_units();
    println!("Break-even point: {} units", break_even);

    Ok(())

}
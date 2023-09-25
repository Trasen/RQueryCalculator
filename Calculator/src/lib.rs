mod calculation_tracker;
mod calculation_resolver;
mod calculables;
mod calculation_hash_tree;
mod calculator;

pub fn calc(query: String) -> String {
    return calculator::calculate(query, calculation_hash_tree::build_calculation_hash_tree());
}

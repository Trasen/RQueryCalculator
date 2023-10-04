pub mod calculation_tracker;
mod calculation_resolver;
pub mod calculables;
pub mod calculation_hash_tree;
mod calculator;

pub fn calc(query: String) -> String {

    if query.len() == 0 {
        return query;
    }

    return String::from_iter(calculator::calculate(query.chars().collect::<Vec<char>>(), calculation_hash_tree::build_calculation_hash_tree()));
}

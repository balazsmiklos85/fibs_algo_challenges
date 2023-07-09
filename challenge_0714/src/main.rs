use crate::{
    island::Island, island_builder::IslandBuilder, island_merger::IslandMerger,
    land_builder::LandBuilder,
};

mod island;
mod island_builder;
mod island_merger;
mod land;
mod land_builder;

fn main() {
    let matrix: Vec<Vec<u32>> = vec![
        vec![0, 1, 1],
        vec![0, 0, 1],
        vec![1, 1, 0]];

    let islands_by_lines: Vec<Vec<Island>> = matrix
        .iter()
        .map(|row| LandBuilder::new(row.clone()))
        .map(|land_builder| land_builder.build())
        .map(|land_masses| IslandBuilder::new(land_masses))
        .map(|island_builder| island_builder.build())
        .collect();
    let complete_islands: Vec<Island> = IslandMerger::new(islands_by_lines)
        .merge();
    let max_area = complete_islands.iter()
        .map(|island| island.area())
        .max();
    match max_area {
        Some(area) => println!("{}", area),
        None => println!("-1"),
    }
}

use std::collections::HashMap;

use crate::island::Island;

pub struct IslandMerger {
    islands_by_lines: Vec<Vec<Island>>,
}

impl IslandMerger {
    pub fn new(islands_by_lines: Vec<Vec<Island>>) -> IslandMerger {
        IslandMerger { islands_by_lines }
    }

    // TODO This is too compliacated to read. It needs better variable names and less nesting.
    pub fn merge(&self) -> Vec<Island> {
        let mut completed_islands = Vec::new();
        let mut previous_islands = Vec::new();

        for islands in &self.islands_by_lines {
            let unconnected = Self::remove_without_neighbours(&mut previous_islands, islands);
            completed_islands.extend_from_slice(&unconnected);

            let connected_islands = Self::remove_connected(&mut previous_islands, islands);
            let merged_islands = Self::merge_connected(&connected_islands);
            previous_islands.extend_from_slice(&merged_islands);

            for previous_island in &mut previous_islands {
                let adjacent_islands = islands
                    .iter()
                    .filter(|island| island.is_adjacent(&previous_island))
                    .map(|island| island.clone())
                    .collect();
                previous_island.join_lands_as_bottom(&adjacent_islands);
            }
            let newly_emerging = Self::select_newly_emerging(&previous_islands, islands);
            previous_islands.extend_from_slice(&newly_emerging);
        }
        completed_islands.extend_from_slice(&previous_islands);
        completed_islands
    }

    fn merge_connected(connected: &HashMap<Island, Vec<Island>>) -> Vec<Island> {
        connected
            .iter()
            .map(|(_island, overlapped)| {
                overlapped
                    .iter()
                    .fold(
                        None,
                        |accumulated: Option<Island>, next_island| match accumulated {
                            Some(result) => Some(result.merge(next_island.clone())),
                            None => Some(next_island.clone()),
                        },
                    )
            })
            .filter(|island| island.is_some())
            .map(|island| island.expect("When an Option is Some, its value should be available."))
            .map(|island| island.clone())
            .collect()
    }

    fn remove_without_neighbours(previous_islands: &Vec<Island>, islands: &Vec<Island>) -> Vec<Island> {
        previous_islands
            .iter()
            .filter(|previous| !islands.iter().any(|island| previous.is_adjacent(island)))
            .map(|island| island.clone())
            .collect()
    }

    fn remove_connected(previous_islands: &mut Vec<Island>, islands: &Vec<Island>) -> HashMap<Island, Vec<Island>> {
        let mut removed = HashMap::new();

        for island in islands {
            let overlapping: Vec<Island> = previous_islands
                .iter()
                .filter(|previous| previous.is_adjacent(island))
                .map(|previous| previous.clone())
                .collect();
            previous_islands.retain(|previous| !overlapping.contains(previous));
            if !overlapping.is_empty() {
                removed.insert(island.clone(), overlapping);
            }
        }
        removed
    }

    fn select_newly_emerging(previous_islands: &Vec<Island>, islands: &Vec<Island>) -> Vec<Island> {
        islands
            .iter()
            .filter(|island| !previous_islands.iter().any(|previous| previous.is_adjacent(island)))
            .map(|island| island.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::land::Land;

    use super::*;

    #[test]
    fn merges_adjacent() {
        let mut land1 = Land::new(1);
        land1.increase();
        land1.increase();
        let mut land2 = Land::new(2);
        land2.increase();
        let island1 = Island::new(land1);
        let island2 = Island::new(land2);
        let islands_by_lines: Vec<Vec<Island>> = vec![
            vec![island1],
            vec![island2],
        ];

        let merger = IslandMerger::new(islands_by_lines);
        let merged = merger.merge();

        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].area(), 5);
    }
}
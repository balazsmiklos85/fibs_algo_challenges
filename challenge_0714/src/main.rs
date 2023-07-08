use std::cmp;

#[derive(Copy, Clone)]
struct SegmentOfLand {
    start: usize,
    end: usize,
}

impl SegmentOfLand {
    fn new(start: usize, end: usize) -> SegmentOfLand {
        SegmentOfLand {
            start: cmp::min(start, end),
            end: cmp::max(start, end),
        }
    }

    fn length(&self) -> usize {
        self.end - self.start + 1
    }

    fn increase(&self) -> SegmentOfLand {
        SegmentOfLand::new(self.start, self.end + 1)
    }

    fn is_overlap(&self, other: &SegmentOfLand) -> bool {
        (self.start >= other.start && self.end <= other.start)
            || (self.start < other.start && self.end >= other.start)
    }
}

#[derive(Clone)]
struct Island {
    last_segments: Vec<SegmentOfLand>,
    area: u64,
}

impl Island {
    fn is_adjacent_with(&self, segment: &SegmentOfLand) -> bool {
        self.last_segments
            .iter()
            .any(|previous_segment| previous_segment.is_overlap(segment))
    }
}

impl PartialEq for Island {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

fn identify_segments_of_land(matrix: Vec<Vec<u8>>) -> Vec<Vec<SegmentOfLand>> {
    let mut result = Vec::new();

    let mut current_segment: Option<SegmentOfLand> = None;
    for row in &matrix {
        let mut current_row = Vec::new();
        for (y, &value) in row.iter().enumerate() {
            if value == 0 {
                if current_segment.is_some() {
                    current_segment = Some(current_segment.unwrap().increase());
                } else {
                    current_segment = Some(SegmentOfLand::new(y, y));
                }
            } else if current_segment.is_some() {
                current_row.push(current_segment.unwrap());
                current_segment = None;
            }
        }
        result.push(current_row);
    }
    result
}

fn merge_islands(islands: &Vec<&Island>) -> Island {
    let mut segments = Vec::new();

    for island in islands {
        let island_segments = island.last_segments.iter().cloned();
        segments.extend(island_segments);
    }

    Island {
        last_segments: segments,
        area: islands.iter().map(|island| island.area).sum::<u64>(),
    }
}

fn replace_islands(
    to_replace: Vec<&Island>,
    replace_with: Island,
    all_islands: &Vec<Island>,
) -> Vec<Island> {
    let mut result = all_islands
        .clone()
        .iter()
        .filter(|island| !to_replace.contains(&island))
        .map(|island| island.clone())
        .collect::<Vec<Island>>();
    result.push(replace_with);
    result
}

fn reduce_to_islands(segments: Vec<Vec<SegmentOfLand>>) -> Vec<Island> {
    let mut result: Vec<Island> = Vec::new();

    for row in &segments {
        let mut new_islands = Vec::new();

        for segment in row {
            let adjacent_islands = result
                .iter()
                .filter(|island| island.is_adjacent_with(segment))
                .collect::<Vec<&Island>>();
            if adjacent_islands.len() > 1 {
                let new_island = merge_islands(&adjacent_islands);
                replace_islands(adjacent_islands, new_island, &result);
            } else if adjacent_islands.len() == 0 {
                new_islands.push(Island {
                    last_segments: vec![*segment],
                    area: segment.length() as u64,
                });
            }
        }
        result = result
            .iter_mut()
            .map(|island| {
                let adjacent_segments = row
                    .iter()
                    .filter(|segment| island.is_adjacent_with(segment))
                    .collect::<Vec<&SegmentOfLand>>();
                let new_last_segments = adjacent_segments
                    .iter()
                    .map(|segment| *segment.clone())
                    .collect::<Vec<SegmentOfLand>>();
                let area = island.area
                    + adjacent_segments
                        .iter()
                        .map(|segment| segment.length() as u64)
                        .sum::<u64>();
                Island {
                    last_segments: new_last_segments,
                    area,
                }
            })
            .collect::<Vec<Island>>();

        result.extend(new_islands);
    }

    result
}

fn main() {
    let matrix: Vec<Vec<u8>> = vec![
        vec![0, 1, 1],
        vec![0, 0, 0],
        vec![1, 1, 0]];

    let segments = identify_segments_of_land(matrix);

    let islands = reduce_to_islands(segments);

    let largest_island_size = islands
        .iter()
        .max_by_key(|island| island.area)
        .map(|island| island.area)
        .unwrap_or(0);

    println!("{}", largest_island_size);
}

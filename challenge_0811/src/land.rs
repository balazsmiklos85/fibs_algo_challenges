#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Land {
    start_index: usize,
    end_index: usize,
}

impl Land {
    pub fn new(index: usize) -> Land {
        Land {
            start_index: index,
            end_index: index,
        }
    }

    pub fn is_adjacent(&self, other: &Land) -> bool {
        (self.start_index <= other.start_index && self.end_index >= other.start_index) ||
            (self.start_index <= other.end_index && self.end_index >= other.end_index)
    }

    pub fn any_adjacent(&self, other_land_masses: &[Land]) -> bool {
        other_land_masses
            .iter()
            .any(|other_land| self.is_adjacent(other_land))
    }

    pub fn increase(&mut self) -> &mut Self {
        self.end_index += 1;
        self
    }

    pub fn size(&self) -> u32 {
        (self.end_index - self.start_index + 1) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn land_adjacent_if_starts_before_previous_ends() {
        let mut land1 = Land::new(1);
        land1.increase();
        land1.increase();
        let mut land2 = Land::new(2);
        land2.increase();
        land2.increase();

        assert!(land2.is_adjacent(&land1));
    }

    #[test]
    fn land_adjacent_if_ends_before_previous_starts() {
        let mut land1 = Land::new(1);
        land1.increase();
        land1.increase();
        let mut land2 = Land::new(2);
        land2.increase();
        land2.increase();

        assert!(land1.is_adjacent(&land2));
    }
}

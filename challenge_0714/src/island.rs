use crate::land::Land;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Island {
    lands: Vec<Land>,
    area: u32,
}

impl Island {
    pub fn new(land: Land) -> Island {
        Island {
            lands: vec![land.clone()],
            area: land.size(),
        }
    }

    pub fn is_adjacent(&self, other: &Island) -> bool {
        self.lands
            .iter()
            .any(|land| land.any_adjacent(&other.lands))
    }

    pub fn merge(mut self, other: Island) -> Island {
        self.lands.extend_from_slice(&other.lands);
        self.area += other.area;
        self
    }

    pub fn area(&self) -> u32 {
        self.area
    }

    pub fn incorporate(&mut self, others: &Vec<Island>) {
        self.lands.clear();
        for other in others {
            self.lands = other.lands.clone();
            self.area += other.area;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn island_takes_area_from_land() {
        let mut land = Land::new(1);
        land.increase();
        let island = Island::new(land.clone());
        
        assert_eq!(island.area(), land.size());
    }

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

    #[test]
    fn island_adjacent_if_land_adjacent() {
        let mut land1 = Land::new(1);
        land1.increase();
        land1.increase();
        let mut land2 = Land::new(2);
        land2.increase();
        land2.increase();
        let island1 = Island::new(land1);
        let island2 = Island::new(land2);

        assert!(island1.is_adjacent(&island2));
    }

    #[test]
    fn merged_island_includes_both_lands() {
        let mut land1 = Land::new(1);
        land1.increase();
        land1.increase();
        let mut land2 = Land::new(2);
        land2.increase();
        land2.increase();
        let island1 = Island::new(land1.clone());
        let island2 = Island::new(land2.clone());

        let merged_island = island1.merge(island2);

        assert_eq!(merged_island.area, land1.size() + land2.size());
    }
    
    #[test]
    fn incorporated_island_no_longer_adjacent_with_previous_land() {
        let land0 = Land::new(1);
        let mut land1 = Land::new(1);
        land1.increase();
        land1.increase();
        let mut land2 = Land::new(2);
        land2.increase();
        land2.increase();
        let originally_adjacent = Island::new(land0);
        let mut island1 = Island::new(land1);
        let island2 = Island::new(land2);
        assert!(island1.is_adjacent(&originally_adjacent));

        island1.incorporate(&vec![island2]);

        assert!(!island1.is_adjacent(&originally_adjacent));
    }
}
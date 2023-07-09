use crate::island::Island;
use crate::land::Land;

pub struct IslandBuilder {
    land_masses: Vec<Land>,
}

impl IslandBuilder {
    pub fn new(land_masses: Vec<Land>) -> IslandBuilder {
        IslandBuilder { land_masses }
    }

    pub fn build(&self) -> Vec<Island> {
        self.land_masses
            .iter()
            .map(|land_mass| Island::new(land_mass.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{island_builder::IslandBuilder, land::Land};

    #[test]
    fn build_adds_both_lands() {
        let land_masses = vec![Land::new(1), Land::new(2)];
        let island_builder = IslandBuilder::new(land_masses);

        let islands = island_builder.build();

        assert_eq!(islands.len(), 2);
    }
}

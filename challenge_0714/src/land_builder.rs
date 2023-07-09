use crate::land::Land;

pub struct LandBuilder {
    values: Vec<u32>,
}

impl LandBuilder {
    pub fn new(values: Vec<u32>) -> LandBuilder {
        LandBuilder { values }
    }

    pub fn build(&self) -> Vec<Land> {
        let mut land_masses = Vec::new();
        let mut current_land_mass: Option<Land> = None;

        for (index, value) in self.values.iter().enumerate() {
            current_land_mass = Self::parse_value_into_land(index, *value, current_land_mass, &mut land_masses);
        }

        match current_land_mass {
            Some(land_mass) => land_masses.push(land_mass),
            None => {}
        }

        land_masses
    }

    fn parse_value_into_land(index: usize, value: u32, current_land_mass: Option<Land>, land_masses: &mut Vec<Land>) -> Option<Land> {
        if value == 0 {
            if let Some(mut current) = current_land_mass {
                current.increase();
                Some(current)
            } else {
                let land_mass = Land::new(index);
                Some(land_mass)
            }
        } else {
            if let Some(current) = current_land_mass {
                land_masses.push(current);
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::land_builder::LandBuilder;

    #[test]
    fn build_finds_both_lands() {
        let land_mass_factory = LandBuilder::new(vec![0, 0, 1, 0]);

        let islands = land_mass_factory.build();

        assert_eq!(islands.len(), 2);
    }

    #[test]
    fn build_detects_land_width() {
        let land_mass_factory = LandBuilder::new(vec![0, 0, 1, 0]);

        let islands = land_mass_factory.build();

        assert_eq!(islands[0].size(), 2);
    }

    #[test]
    fn build_finds_land_at_the_end() {
        let land_mass_factory = LandBuilder::new(vec![0, 0, 1, 0]);

        let islands = land_mass_factory.build();

        assert_eq!(islands[1].size(), 1);
    }
}

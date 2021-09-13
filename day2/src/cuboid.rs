use std::str::FromStr;

pub struct Cuboid(pub usize, pub usize, pub usize);

impl Cuboid {
    fn surface(&self) -> usize {
        2 * self.0 * self.1 + 2 * self.1 * self.2 + 2 * self.0 * self.2
    }

    fn smallest_face(&self) -> usize {
        *[self.0 * self.1, self.1 * self.2, self.0 * self.2]
            .iter()
            .min()
            .unwrap()
    }

    pub fn paper_needed(&self) -> usize {
        self.surface() + self.smallest_face()
    }

    fn smallest_distances_around_sides(&self) -> usize {
        *[
            2 * self.0 + 2 * self.1,
            2 * self.1 + 2 * self.2,
            2 * self.0 + 2 * self.2,
        ]
        .iter()
        .min()
        .unwrap()
    }

    fn volume(&self) -> usize {
        self.0 * self.1 * self.2
    }

    pub fn ribbon_needed(&self) -> usize {
        self.smallest_distances_around_sides() + self.volume()
    }
}

impl FromStr for Cuboid {
    type Err = usize;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .split('x')
            .map(|el| el.parse().unwrap())
            .take(3)
            .collect::<Vec<_>>();
        Ok(Cuboid(split[0], split[1], split[2]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface() {
        assert_eq!(Cuboid(2, 3, 4).surface(), 52);
        assert_eq!(Cuboid(1, 1, 10).surface(), 42);
    }

    #[test]
    fn test_smallest_face() {
        assert_eq!(Cuboid(2, 3, 4).smallest_face(), 6);
        assert_eq!(Cuboid(1, 1, 10).smallest_face(), 1);
    }

    #[test]
    fn test_paper_needed() {
        assert_eq!(Cuboid(2, 3, 4).paper_needed(), 58);
        assert_eq!(Cuboid(1, 1, 10).paper_needed(), 43);
    }

    #[test]
    fn test_smallest_distances_around_sides() {
        assert_eq!(Cuboid(2, 3, 4).smallest_distances_around_sides(), 10);
        assert_eq!(Cuboid(1, 1, 10).smallest_distances_around_sides(), 4);
    }

    #[test]
    fn test_volume() {
        assert_eq!(Cuboid(2, 3, 4).volume(), 24);
        assert_eq!(Cuboid(1, 1, 10).volume(), 10);
    }

    #[test]
    fn test_ribbon_needed() {
        assert_eq!(Cuboid(2, 3, 4).ribbon_needed(), 34);
        assert_eq!(Cuboid(1, 1, 10).ribbon_needed(), 14);
    }
}

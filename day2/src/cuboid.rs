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
}

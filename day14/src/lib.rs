pub struct Reindeer {
    pub name: String,
    speed: usize,
    duration: usize,
    rest: usize,
}

impl Reindeer {
    pub fn new(name: String, speed: usize, duration: usize, rest: usize) -> Self {
        Self {
            name,
            speed,
            duration,
            rest,
        }
    }

    pub fn distance_for(&self, time: usize) -> usize {
        let complete_rush_time = self.duration + self.rest;
        let complete_rush_distance = self.duration * self.speed;

        let repetition = time / complete_rush_time;
        let most_of_the_distance = repetition * complete_rush_distance;
        let time_left = time - complete_rush_time * repetition;

        let rest_of_the_distance = self.duration.min(time_left) * self.speed;

        most_of_the_distance + rest_of_the_distance
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance_for() {
        let comet = Reindeer::new("Comet".into(), 14, 10, 127);
        let dancer = Reindeer::new("Dancer".into(), 16, 11, 162);

        assert_eq!(comet.distance_for(1), 14);
        assert_eq!(dancer.distance_for(1), 16);

        assert_eq!(comet.distance_for(10), 140);
        assert_eq!(dancer.distance_for(10), 160);

        assert_eq!(comet.distance_for(11), 140);
        assert_eq!(dancer.distance_for(11), 176);

        assert_eq!(comet.distance_for(12), 140);
        assert_eq!(dancer.distance_for(12), 176);

        assert_eq!(comet.distance_for(1000), 1120);
        assert_eq!(dancer.distance_for(1000), 1056);
    }
}

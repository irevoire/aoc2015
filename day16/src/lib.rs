use std::ops::{RangeFrom, RangeTo};

#[derive(Debug)]
pub struct Aunt {
    name: String,
    children: Option<usize>,
    cats: Option<RangeFrom<usize>>,
    samoyeds: Option<usize>,
    pomeranians: Option<RangeTo<usize>>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<RangeTo<usize>>,
    trees: Option<RangeFrom<usize>>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl Aunt {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn with_children(&mut self, children: usize) {
        self.children = Some(children);
    }

    pub fn with_cats(&mut self, cats: usize) {
        self.cats = Some(cats..);
    }

    pub fn with_samoyeds(&mut self, samoyeds: usize) {
        self.samoyeds = Some(samoyeds);
    }

    pub fn with_pomeranians(&mut self, pomeranians: usize) {
        self.pomeranians = Some(..pomeranians);
    }

    pub fn with_akitas(&mut self, akitas: usize) {
        self.akitas = Some(akitas);
    }

    pub fn with_vizslas(&mut self, vizslas: usize) {
        self.vizslas = Some(vizslas);
    }

    pub fn with_goldfish(&mut self, goldfish: usize) {
        self.goldfish = Some(..goldfish);
    }

    pub fn with_trees(&mut self, trees: usize) {
        self.trees = Some(trees..);
    }

    pub fn with_cars(&mut self, cars: usize) {
        self.cars = Some(cars);
    }

    pub fn with_perfumes(&mut self, perfumes: usize) {
        self.perfumes = Some(perfumes);
    }

    pub fn could_be(&self, other: &Self) -> bool {
        let mut eq = self.children.and(other.children) == other.children.and(self.children)
            && self.samoyeds.and(other.samoyeds) == other.samoyeds.and(self.samoyeds)
            && self.akitas.and(other.akitas) == other.akitas.and(self.akitas)
            && self.vizslas.and(other.vizslas) == other.vizslas.and(self.vizslas)
            && self.cars.and(other.cars) == other.cars.and(self.cars)
            && self.perfumes.and(other.perfumes) == other.perfumes.and(self.perfumes);

        if self.cats.is_some() && other.cats.is_some() {
            eq &= self
                .cats
                .as_ref()
                .unwrap()
                .contains(&other.cats.as_ref().unwrap().start);
        }
        if self.trees.is_some() && other.trees.is_some() {
            eq &= self
                .trees
                .as_ref()
                .unwrap()
                .contains(&other.trees.as_ref().unwrap().start);
        }
        if self.pomeranians.is_some() && other.pomeranians.is_some() {
            eq &= self
                .pomeranians
                .unwrap()
                .contains(&other.pomeranians.unwrap().end);
        }
        if self.goldfish.is_some() && other.goldfish.is_some() {
            eq &= self
                .goldfish
                .unwrap()
                .contains(&other.goldfish.unwrap().end);
        }
        eq
    }
}

impl PartialEq for Aunt {
    fn eq(&self, other: &Self) -> bool {
        self.children.and(other.children) == other.children.and(self.children)
            && self.cats.as_ref().and(other.cats.as_ref())
                == other.cats.as_ref().and(self.cats.as_ref())
            && self.samoyeds.and(other.samoyeds) == other.samoyeds.and(self.samoyeds)
            && self.pomeranians.and(other.pomeranians) == other.pomeranians.and(self.pomeranians)
            && self.akitas.and(other.akitas) == other.akitas.and(self.akitas)
            && self.vizslas.and(other.vizslas) == other.vizslas.and(self.vizslas)
            && self.goldfish.and(other.goldfish) == other.goldfish.and(self.goldfish)
            && self.trees.as_ref().and(other.trees.as_ref())
                == other.trees.as_ref().and(self.trees.as_ref())
            && self.cars.and(other.cars) == other.cars.and(self.cars)
            && self.perfumes.and(other.perfumes) == other.perfumes.and(self.perfumes)
    }
}

pub fn parse() -> Vec<Aunt> {
    let mut aunts = Vec::new();

    for line in aoc::parser::lines::<String>() {
        let line: Vec<&str> = line.splitn(2, ':').collect();
        let mut aunt = Aunt::new(line[0].into());

        for attribute in line[1].split(',').map(|s| s.split(':')) {
            let attribute: Vec<&str> = attribute.map(str::trim).collect();
            match (attribute[0], attribute[1]) {
                ("children", nb) => aunt.with_children(nb.parse().unwrap()),
                ("cats", nb) => aunt.with_cats(nb.parse().unwrap()),
                ("samoyeds", nb) => aunt.with_samoyeds(nb.parse().unwrap()),
                ("pomeranians", nb) => aunt.with_pomeranians(nb.parse().unwrap()),
                ("akitas", nb) => aunt.with_akitas(nb.parse().unwrap()),
                ("vizslas", nb) => aunt.with_vizslas(nb.parse().unwrap()),
                ("goldfish", nb) => aunt.with_goldfish(nb.parse().unwrap()),
                ("trees", nb) => aunt.with_trees(nb.parse().unwrap()),
                ("cars", nb) => aunt.with_cars(nb.parse().unwrap()),
                ("perfumes", nb) => aunt.with_perfumes(nb.parse().unwrap()),
                (s, _) => panic!("what is {}", s),
            }
        }

        aunts.push(aunt);
    }
    aunts
}

#[derive(Debug)]
pub struct Aunt {
    name: String,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
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
        self.cats = Some(cats);
    }

    pub fn with_samoyeds(&mut self, samoyeds: usize) {
        self.samoyeds = Some(samoyeds);
    }

    pub fn with_pomeranians(&mut self, pomeranians: usize) {
        self.pomeranians = Some(pomeranians);
    }

    pub fn with_akitas(&mut self, akitas: usize) {
        self.akitas = Some(akitas);
    }

    pub fn with_vizslas(&mut self, vizslas: usize) {
        self.vizslas = Some(vizslas);
    }

    pub fn with_goldfish(&mut self, goldfish: usize) {
        self.goldfish = Some(goldfish);
    }

    pub fn with_trees(&mut self, trees: usize) {
        self.trees = Some(trees);
    }

    pub fn with_cars(&mut self, cars: usize) {
        self.cars = Some(cars);
    }

    pub fn with_perfumes(&mut self, perfumes: usize) {
        self.perfumes = Some(perfumes);
    }
}

impl PartialEq for Aunt {
    fn eq(&self, other: &Self) -> bool {
        self.children.and(other.children) == other.children.and(self.children)
            && self.cats.and(other.cats) == other.cats.and(self.cats)
            && self.samoyeds.and(other.samoyeds) == other.samoyeds.and(self.samoyeds)
            && self.pomeranians.and(other.pomeranians) == other.pomeranians.and(self.pomeranians)
            && self.akitas.and(other.akitas) == other.akitas.and(self.akitas)
            && self.vizslas.and(other.vizslas) == other.vizslas.and(self.vizslas)
            && self.goldfish.and(other.goldfish) == other.goldfish.and(self.goldfish)
            && self.trees.and(other.trees) == other.trees.and(self.trees)
            && self.cars.and(other.cars) == other.cars.and(self.cars)
            && self.perfumes.and(other.perfumes) == other.perfumes.and(self.perfumes)
    }
}

pub fn parse() -> Vec<Aunt> {
    let mut aunts = Vec::new();

    for line in aoc::parser::lines_from_args(1) {
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

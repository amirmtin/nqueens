use rand::prelude::*;

#[derive(Debug)]
pub struct Individual {
    pub genes: Vec<usize>,
    pub fitness: usize,
}

impl Individual {
    pub fn calculate_fitness(&mut self) {
        let n = self.genes.len();
        let mut non_attacking_pairs = 0;

        for i in 0..n {
            for j in i + 1..n {
                if self.genes[i] != self.genes[j] &&
                   (self.genes[i] as isize - self.genes[j] as isize).abs() != (i as isize - j as isize).abs() {
                    non_attacking_pairs += 1;
                }
            }
        }
        self.fitness = non_attacking_pairs;
    }
}

pub struct NQueensSolver {
    pub n: usize,
    max_fitness: usize,
    pub population: Vec<Individual>,
    population_size: usize,
    mutation_rate: f64,
    generation_limit: usize,
    rng: ThreadRng,
}

impl NQueensSolver {
    pub fn new(
        n: usize,
        population_size: usize,
        mutation_rate: f64,
        generation_limit: usize,
    ) -> Self {
        let max_fitness = n * (n - 1) / 2;
        let mut rng = rand::thread_rng();
        let population = Self::initial_population(n, &mut rng);
        Self {
            n,
            max_fitness,
            population,
            population_size,
            mutation_rate,
            generation_limit,
            rng,
        }
    }

    fn initial_population(n: usize, rng: &mut ThreadRng) -> Vec<Individual> {
        let mut population = Vec::new();
        for _ in 0..n {
            let mut genes = Vec::new();
            for _ in 0..n {
                genes.push(Self::gen_rand(n, rng));
            }
            let mut individual = Individual { genes, fitness: 0 };
            individual.calculate_fitness();
            population.push(individual);
        }
        population
    }

    fn gen_rand(n: usize, rng: &mut ThreadRng) -> usize {
        rng.gen_range(0..n)
    }
}

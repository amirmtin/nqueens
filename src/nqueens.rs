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
                if self.genes[i] != self.genes[j]
                    && (self.genes[i] as isize - self.genes[j] as isize).abs()
                        != (i as isize - j as isize).abs()
                {
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
}

impl NQueensSolver {
    pub fn new(
        n: usize,
        population_size: usize,
        mutation_rate: f64,
        generation_limit: usize,
    ) -> Self {
        let max_fitness = n * (n - 1) / 2;
        let population = Self::initial_population(n, population_size);
        Self {
            n,
            max_fitness,
            population,
            population_size,
            mutation_rate,
            generation_limit,
        }
    }

    pub fn run(&mut self) -> Option<(Individual, usize)> {
        for generation in 0..self.generation_limit {
            if let Some(solution) = self.select_and_crossover() {
                return Some((solution, generation));
            }
        }
        None
    }

    fn initial_population(n: usize, population_size: usize) -> Vec<Individual> {
        let mut population = Vec::new();
        for _ in 0..population_size {
            let mut genes = Vec::new();
            for _ in 0..n {
                genes.push(Self::generate_random(n));
            }
            let mut individual = Individual { genes, fitness: 0 };
            individual.calculate_fitness();
            population.push(individual);
        }
        population
    }

    fn crossover(&self, parent1: &Individual, parent2: &Individual) -> (Individual, Individual) {
        let crossover_point = Self::generate_random(self.n);
        let mut genes1 = Vec::with_capacity(self.n);
        let mut genes2 = Vec::with_capacity(self.n);

        for i in 0..self.n {
            if i < crossover_point {
                genes1.push(parent1.genes[i]);
                genes2.push(parent2.genes[i]);
            } else {
                genes1.push(parent2.genes[i]);
                genes2.push(parent1.genes[i]);
            }
        }

        let mut child1 = Individual {
            genes: genes1,
            fitness: 0,
        };
        let mut child2 = Individual {
            genes: genes2,
            fitness: 0,
        };
        child1.calculate_fitness();
        child2.calculate_fitness();
        (child1, child2)
    }

    fn mutate(&mut self, individual: &mut Individual) {
        if rand::thread_rng().gen_bool(self.mutation_rate) {
            let mutation_point = Self::generate_random(self.n);
            individual.genes[mutation_point] = Self::generate_random(self.n);
            individual.calculate_fitness();
        }
    }

    fn select_and_crossover(&mut self) -> Option<Individual> {
        let mut new_population = Vec::with_capacity(self.population_size);

        while new_population.len() < self.population_size {
            let parent1 = self.select_individual();
            let parent2 = self.select_individual();

            let (mut child1, mut child2) = self.crossover(parent1, parent2);

            self.mutate(&mut child1);
            self.mutate(&mut child2);

            if child1.fitness == self.max_fitness {
                return Some(child1);
            }
            if child2.fitness == self.max_fitness {
                return Some(child2);
            }

            new_population.push(child1);
            if new_population.len() < self.population_size {
                new_population.push(child2);
            }
        }

        self.population = new_population;
        None
    }

    fn select_individual(&self) -> &Individual {
        let total_fitness: usize = self.sum_fitness();
        let mut cumulative_fitness = 0;
        let target_fitness = Self::generate_random(total_fitness);
        for individual in &self.population {
            cumulative_fitness += individual.fitness;
            if cumulative_fitness > target_fitness {
                return individual;
            }
        }
        &self.population[0]
    }

    fn sum_fitness(&self) -> usize {
        self.population.iter().map(|ind| ind.fitness).sum()
    }

    fn generate_random(upper: usize) -> usize {
        rand::thread_rng().gen_range(0..upper)
    }
}

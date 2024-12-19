use rand::prelude::*;
use rayon::prelude::*;
use std::sync::{Arc, mpsc::{self, Receiver, Sender}};
use std::thread;

#[derive(Debug, Clone)]
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
        Self {
            n,
            max_fitness,
            population_size,
            mutation_rate,
            generation_limit,
        }
    }

    pub fn run(&mut self) -> Option<(Individual, usize)> {
        println!("Max fitness: {}", self.max_fitness);

        let mut population = Arc::new(Self::initial_population(self.n, self.population_size));
        let n = self.n;
        let mutation_rate = self.mutation_rate;

        for generation in 0..self.generation_limit {
            let (select_tx, select_rx) = mpsc::channel();

            let population_clone = Arc::clone(&population);

            thread::spawn(move || {
                Self::select_step(population_clone, select_tx);
            });

            let (crossover_tx, crossover_rx) = mpsc::channel();

            thread::spawn(move || {
                Self::crossover_step(n, select_rx, crossover_tx);
            });

            let (mutate_tx, mutate_rx) = mpsc::channel();

            thread::spawn(move || {
                Self::mutate_step(crossover_rx, mutate_tx, n, mutation_rate);
            });

            let mut new_population = Vec::with_capacity(self.population_size);
            let mut best_fitness = 0;

            for i in mutate_rx {
                if i.fitness == self.max_fitness {
                    return Some((i, generation));
                }
                if i.fitness > best_fitness {
                    best_fitness = i.fitness;
                }
                new_population.push(i);
            }

            println!("Generation {}: Best fitness: {}", generation, best_fitness);
            population = Arc::new(new_population);
        }
        None
    }

    fn select_individual(population: &Vec<Individual>, total_fitness: usize) -> &Individual {
        let mut cumulative_fitness = 0;
        let target_fitness = Self::generate_random(total_fitness);
        for individual in population {
            cumulative_fitness += individual.fitness;
            if cumulative_fitness > target_fitness {
                return individual;
            }
        }
        &population[0]
    }

    fn select_step(population: Arc<Vec<Individual>>, tx: Sender<(Individual, Individual)>) {
        let total_fitness: usize = Self::sum_fitness(&population);
        let population_size = population.len();

        (0..population_size)
            .into_par_iter()
            .step_by(2)
            .for_each(|_| {
                let population_clone = Arc::clone(&population);
                let tx_clone = tx.clone();

                rayon::spawn(move || {
                    let pop1 = Self::select_individual(&population_clone, total_fitness);
                    let pop2 = Self::select_individual(&population_clone, total_fitness);
                    match tx_clone.send((pop1.clone(), pop2.clone())) {
                        Result::Ok(_) => {}
                        Result::Err(_) => {}
                    }
                });
            });
    }

    fn crossover_step(n: usize, rx: Receiver<(Individual, Individual)>, tx: Sender<Individual>) {
        let parents: Vec<(Individual, Individual)> = rx.iter().collect();

        parents.into_par_iter().for_each(|(parent1, parent2)| {
            let (child1, child2) = Self::crossover(n, &parent1, &parent2);
            if let Err(e) = tx.send(child1) {
                eprintln!("Failed to send child1: {}", e);
            }
            if let Err(e) = tx.send(child2) {
                eprintln!("Failed to send child2: {}", e);
            }
        });
    }

    fn mutate_step(rx: Receiver<Individual>, tx: Sender<Individual>, n: usize, mutation_rate: f64) {
        let individuals: Vec<Individual> = rx.iter().collect();

        individuals.into_par_iter().for_each(|mut individual| {
            Self::mutate(&mut individual, n, mutation_rate);
            if let Err(e) = tx.send(individual) {
                eprintln!("Failed to send mutated individual: {}", e);
            }
        });
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

    fn crossover(n: usize, parent1: &Individual, parent2: &Individual) -> (Individual, Individual) {
        let crossover_point = Self::generate_random(n);
        let mut genes1 = Vec::with_capacity(n);
        let mut genes2 = Vec::with_capacity(n);

        for i in 0..n {
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

    fn mutate(individual: &mut Individual, n: usize, mutation_rate: f64) {
        if rand::thread_rng().gen_bool(mutation_rate) {
            let mutation_point = Self::generate_random(n);
            individual.genes[mutation_point] = Self::generate_random(n);
            individual.calculate_fitness();
        }
    }

    fn sum_fitness(population: &Vec<Individual>) -> usize {
        population.iter().map(|ind| ind.fitness).sum()
    }

    fn generate_random(upper: usize) -> usize {
        rand::thread_rng().gen_range(0..upper)
    }
}

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

# N-Queens Solver using Genetic Algorithm in Rust

![Rust](https://img.shields.io/badge/Rust-1.80%2B-blue)
![License](https://img.shields.io/badge/License-MIT-green)


This project implements a **genetic algorithm** to solve the classic **N-Queens problem** in Rust. The solver uses parallelism and multithreading to efficiently find a solution by evolving a population of potential solutions over multiple generations.

The N-Queens problem involves placing `N` queens on an `N x N` chessboard such that no two queens threaten each other. This project demonstrates how genetic algorithms can be used to solve combinatorial optimization problems.

---


## How It Works

1. **Initialization**: A population of random individuals (potential solutions) is generated.
2. **Fitness Calculation**: Each individual's fitness is calculated based on the number of non-attacking queen pairs.
3. **Selection**: Individuals are selected for reproduction based on their fitness.
4. **Crossover**: Selected individuals are combined to produce offspring.
5. **Mutation**: Offspring are mutated to introduce genetic diversity.
6. **Evolution**: The process repeats until a solution with maximum fitness is found or the generation limit is reached.

---

## Building
1. Clone the repository:
   ```bash
   git clone https://github.com/amirmtin/nqueens
   cd nqueens
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

---

## Usage

Run the solver with the following command:

```bash
nqueens --n 8 --population_size 1000 --mutation_rate 0.1 --generation_limit 5000
```

### Command-Line Options

| Option              | Description                              | Default Value |
|---------------------|------------------------------------------|---------------|
| `-n`, `--n`         | Number of queens (N)                     | 8             |
| `-p`, `--population_size` | Size of the population            | 1000          |
| `-m`, `--mutation_rate`   | Mutation rate (probability)       | 0.1           |
| `-g`, `--generation_limit` | Maximum number of generations    | 5000          |

### Example Output

```
Max fitness: 28
Generation 0: Best fitness: 26
Generation 1: Best fitness: 26
Generation 2: Best fitness: 26
...
Generation 48: Best fitness: 26
Generation 49: Best fitness: 26
Generation 50: Best fitness: 28
Solution found in generation 50: [3, 7, 0, 2, 5, 1, 6, 4]
Solution:
.  .  .  ♛  .  .  .  .  

.  .  .  .  .  .  .  ♛  

♛  .  .  .  .  .  .  .  

.  .  ♛  .  .  .  .  .  

.  .  .  .  .  ♛  .  .  

.  ♛  .  .  .  .  .  .  

.  .  .  .  .  .  ♛  .  

.  .  .  .  ♛  .  .  . 
```

---

## Performance

The solver leverages Rust's concurrency features (`rayon` and `mpsc`) to parallelize the selection, crossover, and mutation steps. This significantly improves performance, especially for larger values of `N`.

---

## License

This project is licensed under the MIT License. See [LICENSE](https://github.com/amirmtin/nqueens/blob/main/LICENSE) for details.

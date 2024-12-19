mod nqueens;

fn print_chess_board(solution: &nqueens::Individual, n: usize) {
    println!("Solution:");
    for row in 0..n {
        for col in 0..n {
            if solution.genes[row] == col {
                print!("♛  ");
            } else {
                print!(".  ");
            }
        }
        println!("\n");
    }
    println!();
}

fn main() {
    let n = 8;
    let population_size = 1000;
    let mutation_rate = 0.1;
    let generation_limit = 5000;

    let mut solver = nqueens::NQueensSolver::new(n, population_size, mutation_rate, generation_limit);

    match solver.run() {
        Some((solution, generation)) => {
            println!("Solution found in generation {}: {:?}", generation, solution.genes);
            print_chess_board(&solution, n);
        }
        None => {
            println!("No solution found within the generation limit.");
        }
    }
}

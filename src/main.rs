use clap::Parser;
use nqueens::NQueensSolver;

mod nqueens;

#[derive(Parser, Debug)]
#[command(name = "nqueens-solver")]
#[command(about = "Solves the N-Queens problem using a genetic algorithm.", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8)]
    n: usize,

    #[arg(short, long, default_value_t = 1000)]
    population_size: usize,

    #[arg(short, long, default_value_t = 0.1)]
    mutation_rate: f64,

    #[arg(short, long, default_value_t = 5000)]
    generation_limit: usize,
}

fn main() {
    let args = Args::parse();

    let mut solver = NQueensSolver::new(
        args.n,
        args.population_size,
        args.mutation_rate,
        args.generation_limit,
    );

    match solver.run() {
        Some((solution, generation)) => {
            println!(
                "Solution found in generation {}: {:?}",
                generation, solution.genes
            );
            print_chess_board(&solution, args.n);
        }
        None => {
            println!("No solution found within the generation limit.");
        }
    }
}

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

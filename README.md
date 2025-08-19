Traveling Salesman Problem in Rust

**⚠️ AI Disclosure**: This project includes code generated with assistance from Large Language Models (LLMs) including Claude. All generated code has been reviewed, tested, and validated. Use at your own discretion.







Demonstration of the problem: 
```
10 nodes
=== Solution Found ===
Best path: [0, 4, 8, 9, 7, 1, 3, 5, 2, 6]
Route: 0 -> 4 -> 8 -> 9 -> 7 -> 1 -> 3 -> 5 -> 2 -> 6 -> 0
Total distance: 394.17
Time taken: 0.035 seconds


12 nodes
=== Solution Found ===
Best path: [0, 6, 2, 5, 3, 1, 7, 10, 9, 11, 8, 4]
Route: 0 -> 6 -> 2 -> 5 -> 3 -> 1 -> 7 -> 10 -> 9 -> 11 -> 8 -> 4 -> 0
Total distance: 403.69
Time taken: 4.415 seconds




13 nodes 
=== Solution Found ===
Best path: [0, 4, 8, 11, 12, 10, 9, 7, 1, 3, 5, 2, 6]
Route: 0 -> 4 -> 8 -> 11 -> 12 -> 10 -> 9 -> 7 -> 1 -> 3 -> 5 -> 2 -> 6 -> 0
Total distance: 403.93
Time taken: 55.957 seconds




14 nodes 
=== Solution Found ===
Best path: [0, 6, 2, 13, 5, 3, 1, 7, 9, 10, 12, 11, 8, 4]
Route: 0 -> 6 -> 2 -> 13 -> 5 -> 3 -> 1 -> 7 -> 9 -> 10 -> 12 -> 11 -> 8 -> 4 -> 0
Total distance: 424.12
Time taken: 770.787 seconds



```


















































Prompt: 
Write a very simplified rust command line application
with zero dependencies to generate and solve
using brute force the traveling salesman problem.
The focus of this algorithm should be CORRECTNESS.
The focus of the application should be in zero dependencies.
The application should be easily buildable, runnable, and testable
on Github Actions
so the command line application should accept command line arguments
where it runs and ends automatically after running.
create as few files as possible for easy copy pasting

Claude Opus 4.1: 
I'll create a simple, dependency-free Rust application that generates and
solves the Traveling Salesman Problem using brute force. This will be a
single file that you can easily copy and run.I've created a complete,
zero-dependency Rust implementation of the Traveling Salesman Problem
solver. Here's what the application includes:

## Features:
1. **Zero external dependencies** - Uses only Rust standard library
2. **Brute force algorithm** - Guarantees finding the optimal solution
3. **Command-line interface** - Accepts number of cities and optional
random seed
4. **Automatic execution** - Runs and exits without user interaction
5. **Built-in tests** - Can be run with `cargo test`
6. **Simple random number generator** - For reproducible city generation

## How to use:

### Setup:
1. Create a new Rust project: `cargo new tsp_solver`
2. Replace the contents of `src/main.rs` with the code above
3. Build: `cargo build --release`
4. Run: `cargo run --release 5 42` (5 cities, seed 42)

### Command line usage:
```bash
# Run with 5 cities and default seed (42)
./target/release/tsp_solver 5

# Run with 6 cities and custom seed
./target/release/tsp_solver 6 123
```

### For GitHub Actions:
Create `.github/workflows/test.yml`:

```yaml
name: Test TSP Solver

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - uses: actions-rust-lang/setup-rust-toolchain@v1

    - name: Build
      run: cargo build --release

    - name: Run tests
      run: cargo test

    - name: Test with 5 cities
      run: cargo run --release 5 42

    - name: Test with 7 cities
      run: cargo run --release 7 123
```

## Algorithm Details:
- The algorithm generates all permutations of cities (keeping the first
city fixed to avoid rotational duplicates)
- For each permutation, it calculates the total distance
- It keeps track of the best (shortest) path found
- Time complexity: O(n!) - suitable for up to ~10 cities

## Output:
The program will display:
- City positions
- The optimal path found
- Total distance
- Computation time
- Step-by-step distance breakdown (for small problems)

The focus is entirely on **correctness** - it will always find the truly
optimal solution by checking every possible route.

```rust
use std::env;
use std::process;

#[derive(Debug, Clone)]
struct City {
    id: usize,
    x: f64,
    y: f64,
}

impl City {
    fn distance_to(&self, other: &City) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

struct TSPSolver {
    cities: Vec<City>,
    best_path: Vec<usize>,
    best_distance: f64,
}

impl TSPSolver {
    fn new(cities: Vec<City>) -> Self {
        let initial_path: Vec<usize> = (0..cities.len()).collect();
        TSPSolver {
            cities,
            best_path: initial_path,
            best_distance: f64::INFINITY,
        }
    }

    fn calculate_total_distance(&self, path: &[usize]) -> f64 {
        let mut total = 0.0;
        for i in 0..path.len() {
            let from = &self.cities[path[i]];
            let to = &self.cities[path[(i + 1) % path.len()]];
            total += from.distance_to(to);
        }
        total
    }

    fn permute(&mut self, path: &mut Vec<usize>, l: usize, r: usize) {
        if l == r {
            let distance = self.calculate_total_distance(path);
            if distance < self.best_distance {
                self.best_distance = distance;
                self.best_path = path.clone();
            }
        } else {
            for i in l..=r {
                path.swap(l, i);
                self.permute(path, l + 1, r);
                path.swap(l, i);
            }
        }
    }

    fn solve(&mut self) {
        let n = self.cities.len();
        if n <= 1 {
            return;
        }

        let mut path: Vec<usize> = (0..n).collect();

        // Keep first city fixed to avoid duplicate rotations
        if n > 2 {
            self.permute(&mut path[1..].to_vec(), 0, n - 2);

            // Reconstruct full path with fixed first city
            let mut full_path = vec![0];
            full_path.extend_from_slice(&self.best_path[..n-1]);
            self.best_path = full_path;
        } else {
            self.best_distance = self.calculate_total_distance(&path);
        }
    }

    fn solve_all_permutations(&mut self) {
        let n = self.cities.len();
        if n <= 1 {
            return;
        }

        let mut path: Vec<usize> = (1..n).collect();

        // Generate all permutations of cities 1..n (keeping city 0 fixed)
        self.check_all_permutations(&mut path, 0);

        // Add city 0 at the beginning
        self.best_path.insert(0, 0);
    }

    fn check_all_permutations(&mut self, path: &mut Vec<usize>, start:
usize) {
        if start == path.len() {
            let mut full_path = vec![0];
            full_path.extend_from_slice(path);
            let distance = self.calculate_total_distance(&full_path);
            if distance < self.best_distance {
                self.best_distance = distance;
                self.best_path = path.clone();
            }
            return;
        }

        for i in start..path.len() {
            path.swap(start, i);
            self.check_all_permutations(path, start + 1);
            path.swap(start, i);
        }
    }
}

fn generate_random_cities(n: usize, seed: u64) -> Vec<City> {
    let mut cities = Vec::new();
    let mut rng = SimpleRng::new(seed);

    for i in 0..n {
        cities.push(City {
            id: i,
            x: rng.next_f64() * 100.0,
            y: rng.next_f64() * 100.0,
        });
    }

    cities
}

// Simple linear congruential generator for reproducible randomness
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        SimpleRng {
            state: if seed == 0 { 12345 } else { seed }
        }
    }

    fn next(&mut self) -> u64 {
        self.state =
self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }

    fn next_f64(&mut self) -> f64 {
        (self.next() as f64) / (u64::MAX as f64)
    }
}

fn print_usage() {
    eprintln!("Usage: tsp <num_cities> [seed]");
    eprintln!("  num_cities: Number of cities (1-10 recommended for brute
force)");
    eprintln!("  seed: Optional random seed for city generation (default:
42)");
    eprintln!("\nExample: tsp 5 123");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let num_cities = match args[1].parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Invalid number of cities");
            print_usage();
            process::exit(1);
        }
    };

    if num_cities > 10 {
        eprintln!("Warning: {} cities will take a very long time with brute
force!", num_cities);
        eprintln!("Factorial complexity: {}! permutations to check",
num_cities);
    }

    let seed = if args.len() >= 3 {
        args[2].parse::<u64>().unwrap_or(42)
    } else {
        42
    };

    println!("=== Traveling Salesman Problem Solver ===");
    println!("Cities: {}", num_cities);
    println!("Seed: {}", seed);
    println!();

    // Generate cities
    let cities = generate_random_cities(num_cities, seed);

    // Print city positions
    println!("City Positions:");
    for city in &cities {
        println!("  City {}: ({:.2}, {:.2})", city.id, city.x, city.y);
    }
    println!();

    // Solve TSP
    let mut solver = TSPSolver::new(cities.clone());

    println!("Solving...");
    let start_time = std::time::Instant::now();
    solver.solve_all_permutations();
    let elapsed = start_time.elapsed();

    // Print results
    println!();
    println!("=== Solution Found ===");
    println!("Best path: {:?}", solver.best_path);
    print!("Route: ");
    for (i, &city_id) in solver.best_path.iter().enumerate() {
        if i > 0 {
            print!(" -> ");
        }
        print!("{}", city_id);
    }
    println!(" -> {}", solver.best_path[0]); // Return to start

    println!("Total distance: {:.2}", solver.best_distance);
    println!("Time taken: {:.3} seconds", elapsed.as_secs_f64());

    // Verify solution by printing step-by-step distances
    if num_cities <= 8 {
        println!();
        println!("Distance breakdown:");
        for i in 0..solver.best_path.len() {
            let from_idx = solver.best_path[i];
            let to_idx = solver.best_path[(i + 1) % solver.best_path.len()];
            let from = &cities[from_idx];
            let to = &cities[to_idx];
            let dist = from.distance_to(to);
            println!("  {} -> {}: {:.2}", from_idx, to_idx, dist);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_calculation() {
        let city1 = City { id: 0, x: 0.0, y: 0.0 };
        let city2 = City { id: 1, x: 3.0, y: 4.0 };
        assert_eq!(city1.distance_to(&city2), 5.0);
    }

    #[test]
    fn test_simple_tsp() {
        let cities = vec![
            City { id: 0, x: 0.0, y: 0.0 },
            City { id: 1, x: 1.0, y: 0.0 },
            City { id: 2, x: 1.0, y: 1.0 },
            City { id: 3, x: 0.0, y: 1.0 },
        ];

        let mut solver = TSPSolver::new(cities);
        solver.solve_all_permutations();

        // For a square, the optimal distance should be 4.0
        assert_eq!(solver.best_distance, 4.0);
    }

    #[test]
    fn test_two_cities() {
        let cities = vec![
            City { id: 0, x: 0.0, y: 0.0 },
            City { id: 1, x: 1.0, y: 0.0 },
        ];

        let mut solver = TSPSolver::new(cities);
        solver.solve_all_permutations();

        // Distance should be 2.0 (1.0 each way)
        assert_eq!(solver.best_distance, 2.0);
    }
}
```



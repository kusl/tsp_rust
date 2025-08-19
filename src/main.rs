use std::env;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
    fn solve_parallel(&mut self, num_threads: usize) {
        let n = self.cities.len();
        if n <= 1 {
            return;
        }
       
        if n <= 3 {
            // For very small problems, just use single-threaded
            self.solve_all_permutations();
            return;
        }

        // Create work units by fixing the first two positions
        let mut work_units = Vec::new();
        for i in 1..n {
            for j in 1..n {
                if i != j {
                    let mut prefix = vec![0, i];
                    let mut remaining: Vec<usize> = (1..n)
                        .filter(|&x| x != i && x != j)
                        .collect();
                    remaining.insert(0, j);
                    work_units.push((prefix, remaining));
                }
            }
        }

        // Shared state for best solution
        let best_distance = Arc::new(Mutex::new(f64::INFINITY));
        let best_path = Arc::new(Mutex::new(Vec::new()));
        let cities = Arc::new(self.cities.clone());

        // Divide work among threads
        let chunk_size = (work_units.len() + num_threads - 1) / num_threads;
        let mut handles = Vec::new();

        for chunk in work_units.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let best_distance = Arc::clone(&best_distance);
            let best_path = Arc::clone(&best_path);
            let cities = Arc::clone(&cities);

            let handle = thread::spawn(move || {
                let mut local_best_distance = f64::INFINITY;
                let mut local_best_path = Vec::new();

                for (prefix, mut remaining) in chunk {
                    explore_permutations(
                        &cities,
                        &prefix,
                        &mut remaining,
                        0,
                        &mut local_best_distance,
                        &mut local_best_path,
                    );
                }

                // Update global best if we found something better
                let mut global_best = best_distance.lock().unwrap();
                if local_best_distance < *global_best {
                    *global_best = local_best_distance;
                    *best_path.lock().unwrap() = local_best_path;
                }
            });

            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Update solver's best solution
        self.best_distance = *best_distance.lock().unwrap();
        self.best_path = best_path.lock().unwrap().clone();
    }
}

fn explore_permutations(
    cities: &[City],
    prefix: &[usize],
    remaining: &mut Vec<usize>,
    start: usize,
    best_distance: &mut f64,
    best_path: &mut Vec<usize>,
) {
    if start == remaining.len() {
        let mut full_path = prefix.to_vec();
        full_path.extend_from_slice(remaining);
       
        let distance = calculate_distance(cities, &full_path);
        if distance < *best_distance {
            *best_distance = distance;
            *best_path = full_path;
        }
        return;
    }

    for i in start..remaining.len() {
        remaining.swap(start, i);
        explore_permutations(cities, prefix, remaining, start + 1, best_distance, best_path);
        remaining.swap(start, i);
    }
}

fn calculate_distance(cities: &[City], path: &[usize]) -> f64 {
    let mut total = 0.0;
    for i in 0..path.len() {
        let from = &cities[path[i]];
        let to = &cities[path[(i + 1) % path.len()]];
        total += from.distance_to(to);
    }
    total
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
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }
   
    fn next_f64(&mut self) -> f64 {
        (self.next() as f64) / (u64::MAX as f64)
    }
}

fn print_usage() {
    eprintln!("Usage: tsp <num_cities> [seed] [threads]");
    eprintln!("  num_cities: Number of cities (1-10 recommended for brute force)");
    eprintln!("  seed: Optional random seed for city generation (default: 42)");
    eprintln!("  threads: Optional number of threads for parallel execution (default: number of CPU cores)");
    eprintln!("\nExample: tsp 5 123 4");
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
        eprintln!("Warning: {} cities will take a very long time with brute force!", num_cities);
        eprintln!("Factorial complexity: {}! permutations to check", num_cities);
    }
   
    let seed = if args.len() >= 3 {
        args[2].parse::<u64>().unwrap_or(42)
    } else {
        42
    };

    let num_threads = if args.len() >= 4 {
        args[3].parse::<usize>().unwrap_or_else(|_| {
            thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4)
        })
    } else {
        thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    };
   
    println!("=== Traveling Salesman Problem Solver ===");
    println!("Cities: {}", num_cities);
    println!("Seed: {}", seed);
    println!("Available CPU threads: {}", num_threads);
    println!();
   
    // Generate cities
    let cities = generate_random_cities(num_cities, seed);
   
    // Print city positions
    println!("City Positions:");
    for city in &cities {
        println!("  City {}: ({:.2}, {:.2})", city.id, city.x, city.y);
    }
    println!();
   
    // Solve TSP - Single threaded
    println!("=== Single-threaded Solution ===");
    let mut solver_single = TSPSolver::new(cities.clone());
   
    let start_time = std::time::Instant::now();
    solver_single.solve_all_permutations();
    let elapsed_single = start_time.elapsed();
   
    println!("Best path: {:?}", solver_single.best_path);
    print!("Route: ");
    for (i, &city_id) in solver_single.best_path.iter().enumerate() {
        if i > 0 {
            print!(" -> ");
        }
        print!("{}", city_id);
    }
    println!(" -> {}", solver_single.best_path[0]);
    println!("Total distance: {:.2}", solver_single.best_distance);
    println!("Time taken: {:.3} seconds", elapsed_single.as_secs_f64());
    println!();

    // Solve TSP - Multi-threaded (only if we have enough cities to benefit)
    if num_cities >= 4 {
        println!("=== Multi-threaded Solution ({} threads) ===", num_threads);
        let mut solver_parallel = TSPSolver::new(cities.clone());
       
        let start_time = std::time::Instant::now();
        solver_parallel.solve_parallel(num_threads);
        let elapsed_parallel = start_time.elapsed();
       
        println!("Best path: {:?}", solver_parallel.best_path);
        print!("Route: ");
        for (i, &city_id) in solver_parallel.best_path.iter().enumerate() {
            if i > 0 {
                print!(" -> ");
            }
            print!("{}", city_id);
        }
        println!(" -> {}", solver_parallel.best_path[0]);
        println!("Total distance: {:.2}", solver_parallel.best_distance);
        println!("Time taken: {:.3} seconds", elapsed_parallel.as_secs_f64());
       
        // Verify both methods found the same optimal solution
        if (solver_single.best_distance - solver_parallel.best_distance).abs() > 0.001 {
            eprintln!("\nWARNING: Solutions differ! Single: {:.2}, Parallel: {:.2}",
                     solver_single.best_distance, solver_parallel.best_distance);
        } else {
            println!("\n✓ Both methods found the same optimal solution!");
            let speedup = elapsed_single.as_secs_f64() / elapsed_parallel.as_secs_f64();
            println!("Speedup: {:.2}x", speedup);
        }
    } else {
        println!("(Skipping multi-threaded solution for < 4 cities)");
    }
   
    // Verify solution by printing step-by-step distances
    if num_cities <= 8 {
        println!();
        println!("Distance breakdown:");
        for i in 0..solver_single.best_path.len() {
            let from_idx = solver_single.best_path[i];
            let to_idx = solver_single.best_path[(i + 1) % solver_single.best_path.len()];
    #[test]
    fn test_parallel_correctness() {
        let cities = vec![
            City { id: 0, x: 0.0, y: 0.0 },
            City { id: 1, x: 1.0, y: 0.0 },
            City { id: 2, x: 1.0, y: 1.0 },
            City { id: 3, x: 0.0, y: 1.0 },
            City { id: 4, x: 0.5, y: 0.5 },
        ];
       
        let mut solver_single = TSPSolver::new(cities.clone());
        solver_single.solve_all_permutations();
       
        let mut solver_parallel = TSPSolver::new(cities);
        solver_parallel.solve_parallel(4);
       
        // Both should find the same optimal distance
        assert!((solver_single.best_distance - solver_parallel.best_distance).abs() < 0.001);
    }
}

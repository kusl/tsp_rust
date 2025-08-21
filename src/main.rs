use std::collections::VecDeque;
use std::env;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;

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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn solve(&mut self) {
        let n = self.cities.len();
        if n <= 1 {
            return;
        }

        let path: Vec<usize> = (0..n).collect();

        // Keep first city fixed to avoid duplicate rotations
        if n > 2 {
            self.permute(&mut path[1..].to_vec(), 0, n - 2);

            // Reconstruct full path with fixed first city
            let mut full_path = vec![0];
            full_path.extend_from_slice(&self.best_path[..n - 1]);
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

        // Generate all permutations of cities 1…n (keeping city 0 fixed)
        self.check_all_permutations(&mut path, 0);

        // Add city 0 at the beginning
        self.best_path.insert(0, 0);
    }

    fn check_all_permutations(&mut self, path: &mut Vec<usize>, start: usize) {
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

    fn solve_parallel(&mut self, num_threads: usize) {
        let n = self.cities.len();
        if n <= 1 {
            return;
        }

        if n <= 4 {
            // For very small problems, just use single-threaded
            self.solve_all_permutations();
            return;
        }

        // We'll fix city 0 at position 0, then distribute the work
        // by having each thread handle different starting cities at position 1
        let cities_to_permute: Vec<usize> = (1..n).collect();

        // Shared state for best solution
        let best_distance = Arc::new(Mutex::new(f64::INFINITY));
        let best_path = Arc::new(Mutex::new(Vec::new()));
        let cities = Arc::new(self.cities.clone());

        // Create threads
        let mut handles = Vec::new();
        let chunk_size = cities_to_permute.len().div_ceil(num_threads);

        for chunk in cities_to_permute.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let all_cities = cities_to_permute.clone();
            let best_distance = Arc::clone(&best_distance);
            let best_path = Arc::clone(&best_path);
            let cities = Arc::clone(&cities);

            let handle = thread::spawn(move || {
                let mut local_best_distance = f64::INFINITY;
                let mut local_best_path = Vec::new();

                // For each starting city in our chunk
                for &start_city in &chunk {
                    // Create a path starting with 0 and this city
                    let mut path = vec![0, start_city];

                    // Add remaining cities
                    let mut remaining: Vec<usize> = all_cities
                        .iter()
                        .filter(|&&c| c != start_city)
                        .cloned()
                        .collect();

                    // Generate all permutations with this fixed start
                    permute_and_check(
                        &cities,
                        &mut path,
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
                    let mut global_path = best_path.lock().unwrap();
                    *global_path = local_best_path;
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

// New optimized solver with distance matrix and branch-and-bound
struct OptimizedTSPSolver {
    cities: Vec<City>,
    distance_matrix: Vec<Vec<f64>>,
    best_path: Vec<usize>,
    best_distance: f64,
}

impl OptimizedTSPSolver {
    fn new(cities: Vec<City>) -> Self {
        let n = cities.len();
        let mut distance_matrix = vec![vec![0.0; n]; n];

        // Pre-calculate all distances (optimization 1: distance matrix caching)
        for i in 0..n {
            for j in i + 1..n {
                let dist = cities[i].distance_to(&cities[j]);
                distance_matrix[i][j] = dist;
                distance_matrix[j][i] = dist;
            }
        }

        let initial_path: Vec<usize> = (0..cities.len()).collect();
        OptimizedTSPSolver {
            cities,
            distance_matrix,
            best_path: initial_path,
            best_distance: f64::INFINITY,
        }
    }

    #[allow(dead_code)]
    fn calculate_total_distance(&self, path: &[usize]) -> f64 {
        let mut total = 0.0;
        for i in 0..path.len() {
            let from = path[i];
            let to = path[(i + 1) % path.len()];
            total += self.distance_matrix[from][to]; // O(1) lookup instead of calculation
        }
        total
    }

    // Optimization 2: Branch and bound with early termination
    fn permute_with_bound(&mut self, path: &mut Vec<usize>, l: usize, r: usize, current_dist: f64) {
        // Early termination if partial distance already exceeds best
        if current_dist >= self.best_distance {
            return;
        }

        if l == r {
            // Complete the cycle by adding distance back to start
            let total_distance = current_dist + self.distance_matrix[path[r]][path[0]];
            if total_distance < self.best_distance {
                self.best_distance = total_distance;
                self.best_path = path.clone();
            }
        } else {
            for i in l..=r {
                path.swap(l, i);

                // Calculate incremental distance for branch and bound
                let new_dist = if l == 0 {
                    0.0
                } else {
                    current_dist + self.distance_matrix[path[l - 1]][path[l]]
                };

                self.permute_with_bound(path, l + 1, r, new_dist);
                path.swap(l, i);
            }
        }
    }

    fn solve_optimized(&mut self) {
        let n = self.cities.len();
        if n <= 1 {
            return;
        }

        if n <= 20 {
            // Use dynamic programming with bitmask for small problems (optimization 3)
            self.solve_with_bitmask();
        } else {
            // Use branch and bound for larger problems
            let mut path: Vec<usize> = (1..n).collect();
            self.permute_with_bound(&mut path, 0, n - 2, 0.0);

            // Add city 0 at the beginning
            self.best_path.insert(0, 0);
        }
    }

    // Optimization 3: Dynamic programming with bitmask for exact solution
    fn solve_with_bitmask(&mut self) {
        let n = self.cities.len();
        if n > 20 {
            // Fall back to branch and bound for larger problems
            let mut path: Vec<usize> = (1..n).collect();
            self.permute_with_bound(&mut path, 0, n - 2, 0.0);
            self.best_path.insert(0, 0);
            return;
        }

        let mut dp = vec![vec![f64::INFINITY; 1 << n]; n];
        let mut parent = vec![vec![None; 1 << n]; n];

        // Start at city 0
        dp[0][1] = 0.0;

        for mask in 1..(1 << n) {
            for u in 0..n {
                if (mask & (1 << u)) == 0 {
                    continue;
                }

                for v in 0..n {
                    if u == v || (mask & (1 << v)) != 0 {
                        continue;
                    }

                    let new_mask = mask | (1 << v);
                    let new_dist = dp[u][mask] + self.distance_matrix[u][v];

                    if new_dist < dp[v][new_mask] {
                        dp[v][new_mask] = new_dist;
                        parent[v][new_mask] = Some(u);
                    }
                }
            }
        }

        // Find minimum cost to return to start
        let final_mask = (1 << n) - 1;
        self.best_distance = f64::INFINITY;
        let mut last_city = 0;
        #[allow(clippy::needless_range_loop)]
        for i in 1..n {
            let total = dp[i][final_mask] + self.distance_matrix[i][0];
            if total < self.best_distance {
                self.best_distance = total;
                last_city = i;
            }
        }

        // Reconstruct path
        self.reconstruct_path(parent, final_mask, last_city);
    }

    fn reconstruct_path(
        &mut self,
        parent: Vec<Vec<Option<usize>>>,
        mut mask: usize,
        mut current: usize,
    ) {
        let mut path = Vec::new();

        while let Some(prev) = parent[current][mask] {
            path.push(current);
            mask ^= 1 << current;
            current = prev;
        }

        path.push(0); // Start city
        path.reverse();
        self.best_path = path;
    }
}

// Optimization 4: Memory pool for reducing allocations (used in parallel version)
#[allow(dead_code)]
struct PathPool {
    pool: Arc<Mutex<VecDeque<Vec<usize>>>>,
}

#[allow(dead_code)]
impl PathPool {
    fn new(capacity: usize, path_length: usize) -> Self {
        let mut pool = VecDeque::with_capacity(capacity);
        for _ in 0..capacity {
            pool.push_back(vec![0; path_length]);
        }
        PathPool {
            pool: Arc::new(Mutex::new(pool)),
        }
    }

    fn get(&self) -> Option<Vec<usize>> {
        self.pool.lock().unwrap().pop_front()
    }

    fn return_path(&self, mut path: Vec<usize>) {
        path.clear();
        if let Ok(mut pool) = self.pool.lock() {
            if pool.len() < pool.capacity() {
                pool.push_back(path);
            }
        }
    }
}

fn permute_and_check(
    cities: &[City],
    prefix: &mut Vec<usize>,
    remaining: &mut Vec<usize>,
    start_idx: usize,
    best_distance: &mut f64,
    best_path: &mut Vec<usize>,
) {
    if start_idx == remaining.len() {
        // Complete path
        let mut full_path = prefix.clone();
        full_path.extend_from_slice(remaining);

        let distance = calculate_distance(cities, &full_path);
        if distance < *best_distance {
            *best_distance = distance;
            *best_path = full_path;
        }
        return;
    }

    for i in start_idx..remaining.len() {
        remaining.swap(start_idx, i);
        permute_and_check(
            cities,
            prefix,
            remaining,
            start_idx + 1,
            best_distance,
            best_path,
        );
        remaining.swap(start_idx, i);
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
            state: if seed == 0 { 12345 } else { seed },
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
    eprintln!("Usage: tsp <num_cities> [seed] [threads] [--all]");
    eprintln!("  num_cities: Number of cities");
    eprintln!("  seed: Optional random seed for city generation (default: 42)");
    eprintln!("  threads: Optional number of threads for parallel execution (default: number of CPU cores)");
    eprintln!("  --all: Run all implementations (by default, only optimized solution runs for 15+ cities)");
    eprintln!("\nExample: tsp 5 123 4");
    eprintln!("Example: tsp 16 42 --all");
    eprintln!("\nNote: For 15+ cities, only the optimized solution runs by default.");
    eprintln!("      Use --all flag to run all implementations (may take very long!)");
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

    // Check for --all flag
    let run_all = args.iter().any(|arg| arg == "--all");

    // Parse seed (skip --all if it appears in position 2)
    let seed = if args.len() >= 3 && args[2] != "--all" {
        args[2].parse::<u64>().unwrap_or(42)
    } else {
        42
    };

    // Parse threads (skip --all if it appears in position 3)
    let num_threads = if args.len() >= 4 && args[3] != "--all" {
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

    // Determine whether to run all implementations
    let should_run_all = run_all || num_cities < 15;

    if num_cities >= 15 && !run_all {
        println!(
            "Note: For {} cities, only running optimized solution.",
            num_cities
        );
        println!("      Use --all flag to run all implementations (warning: may take very long!)");
        println!();
    }

    if num_cities > 15 && run_all {
        eprintln!(
            "Warning: {} cities with all implementations will take a very long time!",
            num_cities
        );
        eprintln!(
            "Factorial complexity: {}! permutations for brute force",
            num_cities
        );
        eprintln!("Consider running without --all flag for optimized solution only.");
        println!();
    }

    println!("=== Traveling Salesman Problem Solver ===");
    println!("Cities: {}", num_cities);
    println!("Seed: {}", seed);
    if should_run_all {
        println!("Mode: All implementations");
        println!("Available CPU threads: {}", num_threads);
    } else {
        println!("Mode: Optimized solution only");
    }
    println!();

    // Generate cities
    let cities = generate_random_cities(num_cities, seed);

    // Print city positions
    println!("City Positions:");
    for city in &cities {
        println!("  City {}: ({:.2}, {:.2})", city.id, city.x, city.y);
    }
    println!();

    if should_run_all {
        // Run all implementations
        run_all_implementations(cities, seed, num_threads, num_cities);
    } else {
        // Run only optimized solution for 15+ cities
        println!("=== Optimized Solution (Distance Matrix + Branch & Bound + Bitmask DP) ===");
        let mut solver_optimized = OptimizedTSPSolver::new(cities.clone());

        let start_time = std::time::Instant::now();
        solver_optimized.solve_optimized();
        let elapsed_optimized = start_time.elapsed();

        println!("Best path: {:?}", solver_optimized.best_path);
        print!("Route: ");
        for (i, &city_id) in solver_optimized.best_path.iter().enumerate() {
            if i > 0 {
                print!(" -> ");
            }
            print!("{}", city_id);
        }
        println!(" -> {}", solver_optimized.best_path[0]);
        println!("Total distance: {:.2}", solver_optimized.best_distance);
        println!("Time taken: {:.3} seconds", elapsed_optimized.as_secs_f64());

        // Verify solution by printing step-by-step distances for small problems
        if num_cities <= 8 {
            println!();
            println!("Distance breakdown:");
            let path = &solver_optimized.best_path;
            for i in 0..path.len() {
                let from_idx = path[i];
                let to_idx = path[(i + 1) % path.len()];
                let from = &cities[from_idx];
                let to = &cities[to_idx];
                let dist = from.distance_to(to);
                println!("  {} -> {}: {:.2}", from_idx, to_idx, dist);
            }
        }
    }
}

fn run_all_implementations(cities: Vec<City>, seed: u64, num_threads: usize, num_cities: usize) {
    // Create a random order for running the three implementations
    let mut rng = SimpleRng::new(seed + 1000); // Different seed for randomization
    let mut order = vec![0, 1, 2]; // 0: single, 1: parallel, 2: optimized

    // Simple shuffle
    for i in (1..order.len()).rev() {
        let j = (rng.next() as usize) % (i + 1);
        order.swap(i, j);
    }

    let mut results = vec![
        (
            String::new(),
            f64::INFINITY,
            std::time::Duration::ZERO,
            Vec::new()
        );
        3
    ];

    for &implementation in &order {
        match implementation {
            0 => {
                // Single-threaded implementation
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

                results[0] = (
                    "Single-threaded".to_string(),
                    solver_single.best_distance,
                    elapsed_single,
                    solver_single.best_path,
                );
            }
            1 => {
                // Multithreaded implementation (only if we have enough cities)
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
                    println!();

                    results[1] = (
                        "Multi-threaded".to_string(),
                        solver_parallel.best_distance,
                        elapsed_parallel,
                        solver_parallel.best_path,
                    );
                } else {
                    println!("=== Multi-threaded Solution (Skipped for < 4 cities) ===");
                    println!();
                    results[1] = (
                        "Multi-threaded (Skipped)".to_string(),
                        f64::INFINITY,
                        std::time::Duration::ZERO,
                        Vec::new(),
                    );
                }
            }
            2 => {
                // Optimized implementation
                println!(
                    "=== Optimized Solution (Distance Matrix + Branch & Bound + Bitmask DP) ==="
                );
                let mut solver_optimized = OptimizedTSPSolver::new(cities.clone());

                let start_time = std::time::Instant::now();
                solver_optimized.solve_optimized();
                let elapsed_optimized = start_time.elapsed();

                println!("Best path: {:?}", solver_optimized.best_path);
                print!("Route: ");
                for (i, &city_id) in solver_optimized.best_path.iter().enumerate() {
                    if i > 0 {
                        print!(" -> ");
                    }
                    print!("{}", city_id);
                }
                println!(" -> {}", solver_optimized.best_path[0]);
                println!("Total distance: {:.2}", solver_optimized.best_distance);
                println!("Time taken: {:.3} seconds", elapsed_optimized.as_secs_f64());
                println!();

                results[2] = (
                    "Optimized".to_string(),
                    solver_optimized.best_distance,
                    elapsed_optimized,
                    solver_optimized.best_path,
                );
            }
            _ => unreachable!(),
        }
    }

    // Summary comparison
    println!("=== Performance Summary ===");
    println!(
        "Implementation order: {:?}",
        order
            .iter()
            .map(|&i| match i {
                0 => "Single",
                1 => "Multi",
                2 => "Optimized",
                _ => "Unknown",
            })
            .collect::<Vec<_>>()
    );
    println!();

    // Find best distance across all implementations
    let best_distance = results
        .iter()
        .filter(|(_, dist, _, _)| *dist != f64::INFINITY)
        .map(|(_, dist, _, _)| *dist)
        .fold(f64::INFINITY, f64::min);

    // Sort results by time for performance comparison
    let mut perf_results = results.clone();
    perf_results.sort_by(|a, b| {
        if a.1 == f64::INFINITY && b.1 == f64::INFINITY {
            std::cmp::Ordering::Equal
        } else if a.1 == f64::INFINITY {
            std::cmp::Ordering::Greater
        } else if b.1 == f64::INFINITY {
            std::cmp::Ordering::Less
        } else {
            a.2.cmp(&b.2)
        }
    });

    println!("Performance Ranking:");
    for (rank, (name, distance, time, _)) in perf_results.iter().enumerate() {
        if *distance != f64::INFINITY {
            let speedup = if rank == 0 {
                1.0
            } else {
                time.as_secs_f64() / perf_results[0].2.as_secs_f64()
            };
            println!(
                "  {}. {}: {:.3}s (distance: {:.2}, {}x slower)",
                rank + 1,
                name,
                time.as_secs_f64(),
                distance,
                if rank == 0 {
                    "baseline".to_string()
                } else {
                    format!("{:.2}", speedup)
                }
            );
        } else {
            println!("  {}. {}: skipped", rank + 1, name);
        }
    }

    // Verify all implementations found the same optimal solution
    let valid_results: Vec<_> = results
        .iter()
        .filter(|(_, dist, _, _)| *dist != f64::INFINITY)
        .collect();

    if valid_results.len() > 1 {
        let all_same = valid_results
            .iter()
            .all(|(_, dist, _, _)| (dist - best_distance).abs() < 0.001);

        if all_same {
            println!("\n✓ All implementations found the same optimal solution!");
        } else {
            println!("\n⚠️  WARNING: Implementations found different solutions!");
            for (name, distance, _, _) in &valid_results {
                println!("  {}: {:.2}", name, distance);
            }
        }
    }

    // Verify solution by printing step-by-step distances for small problems
    if num_cities <= 8 && !results[0].3.is_empty() {
        println!();
        println!("Distance breakdown (using single-threaded result):");
        let path = &results[0].3;
        for i in 0..path.len() {
            let from_idx = path[i];
            let to_idx = path[(i + 1) % path.len()];
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
        let city1 = City {
            id: 0,
            x: 0.0,
            y: 0.0,
        };
        let city2 = City {
            id: 1,
            x: 3.0,
            y: 4.0,
        };
        assert_eq!(city1.distance_to(&city2), 5.0);
    }

    #[test]
    fn test_simple_tsp() {
        let cities = vec![
            City {
                id: 0,
                x: 0.0,
                y: 0.0,
            },
            City {
                id: 1,
                x: 1.0,
                y: 0.0,
            },
            City {
                id: 2,
                x: 1.0,
                y: 1.0,
            },
            City {
                id: 3,
                x: 0.0,
                y: 1.0,
            },
        ];

        let mut solver = TSPSolver::new(cities);
        solver.solve_all_permutations();

        // For a square, the optimal distance should be 4.0
        assert_eq!(solver.best_distance, 4.0);
    }

    #[test]
    fn test_optimized_tsp() {
        let cities = vec![
            City {
                id: 0,
                x: 0.0,
                y: 0.0,
            },
            City {
                id: 1,
                x: 1.0,
                y: 0.0,
            },
            City {
                id: 2,
                x: 1.0,
                y: 1.0,
            },
            City {
                id: 3,
                x: 0.0,
                y: 1.0,
            },
        ];

        let mut solver = OptimizedTSPSolver::new(cities);
        solver.solve_optimized();

        // For a square, the optimal distance should be 4.0
        assert_eq!(solver.best_distance, 4.0);
    }

    #[test]
    fn test_two_cities() {
        let cities = vec![
            City {
                id: 0,
                x: 0.0,
                y: 0.0,
            },
            City {
                id: 1,
                x: 1.0,
                y: 0.0,
            },
        ];

        let mut solver = TSPSolver::new(cities);
        solver.solve_all_permutations();

        // Distance should be 2.0 (1.0 each way)
        assert_eq!(solver.best_distance, 2.0);
    }

    #[test]
    fn test_parallel_correctness() {
        let cities = vec![
            City {
                id: 0,
                x: 0.0,
                y: 0.0,
            },
            City {
                id: 1,
                x: 1.0,
                y: 0.0,
            },
            City {
                id: 2,
                x: 1.0,
                y: 1.0,
            },
            City {
                id: 3,
                x: 0.0,
                y: 1.0,
            },
            City {
                id: 4,
                x: 0.5,
                y: 0.5,
            },
        ];

        let mut solver_single = TSPSolver::new(cities.clone());
        solver_single.solve_all_permutations();

        let mut solver_parallel = TSPSolver::new(cities.clone());
        solver_parallel.solve_parallel(4);

        let mut solver_optimized = OptimizedTSPSolver::new(cities);
        solver_optimized.solve_optimized();

        // All should find the same optimal distance
        assert!((solver_single.best_distance - solver_parallel.best_distance).abs() < 0.001);
        assert!((solver_single.best_distance - solver_optimized.best_distance).abs() < 0.001);
    }
}

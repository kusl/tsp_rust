Traveling Salesman Problem in Rust









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










**⚠️ AI Disclosure**: This project includes code generated with assistance from Large Language Models (LLMs) including Claude. All generated code has been reviewed, tested, and validated. Use at your own discretion.









```
kushal@flex2024:~/src/rust/tsp_rust$ cargo fmt --all -- --check
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:122:
         if n <= 1 {
             return;
         }
-        
+
         if n <= 4 {
             // For very small problems, just use single-threaded
             self.solve_all_permutations();
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:132:
         // We'll fix city 0 at position 0, then distribute the work
         // by having each thread handle different starting cities at position 1
         let cities_to_permute: Vec<usize> = (1..n).collect();
-        
+
         // Shared state for best solution
         let best_distance = Arc::new(Mutex::new(f64::INFINITY));
         let best_path = Arc::new(Mutex::new(Vec::new()));
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:141:
         // Create threads
         let mut handles = Vec::new();
         let chunk_size = cities_to_permute.len().div_ceil(num_threads);
-        
+
         for chunk in cities_to_permute.chunks(chunk_size) {
             let chunk = chunk.to_vec();
             let all_cities = cities_to_permute.clone();
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:157:
                 for &start_city in &chunk {
                     // Create a path starting with 0 and this city
                     let mut path = vec![0, start_city];
-                    
+
                     // Add remaining cities
                     let mut remaining: Vec<usize> = all_cities
                         .iter()
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:164:
                         .filter(|&&c| c != start_city)
                         .cloned()
                         .collect();
-                    
+
                     // Generate all permutations with this fixed start
                     permute_and_check(
                         &cities,
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:211:
         // Complete path
         let mut full_path = prefix.clone();
         full_path.extend_from_slice(remaining);
-        
+
         let distance = calculate_distance(cities, &full_path);
         if distance < *best_distance {
             *best_distance = distance;
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:222:
 
     for i in start_idx..remaining.len() {
         remaining.swap(start_idx, i);
-        permute_and_check(cities, prefix, remaining, start_idx + 1, best_distance, best_path);
+        permute_and_check(
+            cities,
+            prefix,
+            remaining,
+            start_idx + 1,
+            best_distance,
+            best_path,
+        );
         remaining.swap(start_idx, i);
     }
 }
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:240:
 fn generate_random_cities(n: usize, seed: u64) -> Vec<City> {
     let mut cities = Vec::new();
     let mut rng = SimpleRng::new(seed);
-    
+
     for i in 0..n {
         cities.push(City {
             id: i,
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:248:
             y: rng.next_f64() * 100.0,
         });
     }
-    
+
     cities
 }
 
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:259:
 
 impl SimpleRng {
     fn new(seed: u64) -> Self {
-        SimpleRng { 
-            state: if seed == 0 { 12345 } else { seed }
+        SimpleRng {
+            state: if seed == 0 { 12345 } else { seed },
         }
     }
-    
+
     fn next(&mut self) -> u64 {
         self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
         self.state
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:270:
     }
-    
+
     fn next_f64(&mut self) -> f64 {
         (self.next() as f64) / (u64::MAX as f64)
     }
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:284:
 
 fn main() {
     let args: Vec<String> = env::args().collect();
-    
+
     if args.len() < 2 {
         print_usage();
         process::exit(1);
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:291:
     }
-    
+
     let num_cities = match args[1].parse::<usize>() {
         Ok(n) => n,
         Err(_) => {
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:298:
             process::exit(1);
         }
     };
-    
+
     if num_cities > 10 {
-        eprintln!("Warning: {} cities will take a very long time with brute force!", num_cities);
-        eprintln!("Factorial complexity: {}! permutations to check", num_cities);
+        eprintln!(
+            "Warning: {} cities will take a very long time with brute force!",
+            num_cities
+        );
+        eprintln!(
+            "Factorial complexity: {}! permutations to check",
+            num_cities
+        );
     }
-    
+
     let seed = if args.len() >= 3 {
         args[2].parse::<u64>().unwrap_or(42)
     } else {
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:321:
             .map(|n| n.get())
             .unwrap_or(4)
     };
-    
+
     println!("=== Traveling Salesman Problem Solver ===");
     println!("Cities: {}", num_cities);
     println!("Seed: {}", seed);
Diff in /home/kushal/src/rust/tsp_rust/src/main.rs:328:
     println!("Available CPU threads: {}", num_threads);
     println!();
-    
+
     // Generate cities
     let cities = generate_random_cities(num_cities, seed);
-    
+
     // Print city positions
     println!("City Positions:");
     for city in &cities {
kushal@flex2024:~/src/rust/tsp_rust$ cargo fmt --all
kushal@flex2024:~/src/rust/tsp_rust$ time cargo run --release 10 42
   Compiling tsp_rust v0.1.0 (/home/kushal/src/rust/tsp_rust)
    Finished `release` profile [optimized] target(s) in 1.01s
     Running `target/release/tsp_rust 10 42`
=== Traveling Salesman Problem Solver ===
Cities: 10
Seed: 42
Available CPU threads: 8

City Positions:
  City 0: (0.00, 0.01)
  City 1: (78.56, 60.47)
  City 2: (81.05, 1.19)
  City 3: (70.41, 37.51)
  City 4: (28.28, 37.96)
  City 5: (63.30, 31.49)
  City 6: (41.54, 8.58)
  City 7: (99.72, 80.97)
  City 8: (29.60, 45.77)
  City 9: (0.91, 96.23)

=== Single-threaded Solution ===
Best path: [0, 4, 8, 9, 7, 1, 3, 5, 2, 6]
Route: 0 -> 4 -> 8 -> 9 -> 7 -> 1 -> 3 -> 5 -> 2 -> 6 -> 0
Total distance: 394.17
Time taken: 0.045 seconds

=== Multi-threaded Solution (8 threads) ===
Best path: [0, 6, 2, 5, 3, 1, 7, 9, 8, 4]
Route: 0 -> 6 -> 2 -> 5 -> 3 -> 1 -> 7 -> 9 -> 8 -> 4 -> 0
Total distance: 394.17
Time taken: 0.020 seconds

✓ Both methods found the same optimal solution!
Speedup: 2.20x

real	0m1.158s
user	0m1.179s
sys	0m0.507s
kushal@flex2024:~/src/rust/tsp_rust$ time cargo run --release 12 42
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/tsp_rust 12 42`
Warning: 12 cities will take a very long time with brute force!
Factorial complexity: 12! permutations to check
=== Traveling Salesman Problem Solver ===
Cities: 12
Seed: 42
Available CPU threads: 8

City Positions:
  City 0: (0.00, 0.01)
  City 1: (78.56, 60.47)
  City 2: (81.05, 1.19)
  City 3: (70.41, 37.51)
  City 4: (28.28, 37.96)
  City 5: (63.30, 31.49)
  City 6: (41.54, 8.58)
  City 7: (99.72, 80.97)
  City 8: (29.60, 45.77)
  City 9: (0.91, 96.23)
  City 10: (24.66, 76.52)
  City 11: (24.11, 45.28)

=== Single-threaded Solution ===
Best path: [0, 6, 2, 5, 3, 1, 7, 10, 9, 11, 8, 4]
Route: 0 -> 6 -> 2 -> 5 -> 3 -> 1 -> 7 -> 10 -> 9 -> 11 -> 8 -> 4 -> 0
Total distance: 403.69
Time taken: 4.589 seconds

=== Multi-threaded Solution (8 threads) ===
Best path: [0, 6, 2, 5, 3, 1, 7, 10, 9, 11, 8, 4]
Route: 0 -> 6 -> 2 -> 5 -> 3 -> 1 -> 7 -> 10 -> 9 -> 11 -> 8 -> 4 -> 0
Total distance: 403.69
Time taken: 1.353 seconds

✓ Both methods found the same optimal solution!
Speedup: 3.39x

real	0m6.024s
user	0m11.485s
sys	0m0.037s
kushal@flex2024:~/src/rust/tsp_rust$ time cargo run --release 13 42
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/tsp_rust 13 42`
Warning: 13 cities will take a very long time with brute force!
Factorial complexity: 13! permutations to check
=== Traveling Salesman Problem Solver ===
Cities: 13
Seed: 42
Available CPU threads: 8

City Positions:
  City 0: (0.00, 0.01)
  City 1: (78.56, 60.47)
  City 2: (81.05, 1.19)
  City 3: (70.41, 37.51)
  City 4: (28.28, 37.96)
  City 5: (63.30, 31.49)
  City 6: (41.54, 8.58)
  City 7: (99.72, 80.97)
  City 8: (29.60, 45.77)
  City 9: (0.91, 96.23)
  City 10: (24.66, 76.52)
  City 11: (24.11, 45.28)
  City 12: (22.73, 63.63)

=== Single-threaded Solution ===
Best path: [0, 4, 8, 11, 12, 10, 9, 7, 1, 3, 5, 2, 6]
Route: 0 -> 4 -> 8 -> 11 -> 12 -> 10 -> 9 -> 7 -> 1 -> 3 -> 5 -> 2 -> 6 -> 0
Total distance: 403.93
Time taken: 55.959 seconds

=== Multi-threaded Solution (8 threads) ===
Best path: [0, 6, 2, 5, 3, 1, 7, 9, 10, 12, 11, 8, 4]
Route: 0 -> 6 -> 2 -> 5 -> 3 -> 1 -> 7 -> 9 -> 10 -> 12 -> 11 -> 8 -> 4 -> 0
Total distance: 403.93
Time taken: 15.452 seconds

✓ Both methods found the same optimal solution!
Speedup: 3.62x

real	1m11.505s
user	2m25.218s
sys	0m0.086s
kushal@flex2024:~/src/rust/tsp_rust$ 
```
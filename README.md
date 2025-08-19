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
kushal@flex2024:~/src/rust/tsp_rust$ time cargo run --release 10 42
   Compiling tsp_rust v0.1.0 (/home/kushal/src/rust/tsp_rust)
    Finished `release` profile [optimized] target(s) in 0.63s
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
Time taken: 0.040 seconds

=== Multi-threaded Solution (8 threads) ===
Best path: [0, 4, 8, 9, 7, 1, 3, 5, 2, 6]
Route: 0 -> 4 -> 8 -> 9 -> 7 -> 1 -> 3 -> 5 -> 2 -> 6 -> 0
Total distance: 394.17
Time taken: 0.648 seconds

✓ Both methods found the same optimal solution!
Speedup: 0.06x

real	0m1.390s
user	0m2.341s
sys	0m2.258s
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
Time taken: 48.396 seconds

=== Multi-threaded Solution (8 threads) ===
Best path: [0, 4, 8, 11, 12, 10, 9, 7, 1, 3, 5, 2, 6]
Route: 0 -> 4 -> 8 -> 11 -> 12 -> 10 -> 9 -> 7 -> 1 -> 3 -> 5 -> 2 -> 6 -> 0
Total distance: 403.93
Time taken: 1352.130 seconds

✓ Both methods found the same optimal solution!
Speedup: 0.04x

real	23m20.593s
user	41m44.955s
sys	74m49.482s
kushal@flex2024:~/src/rust/tsp_rust$ 
```
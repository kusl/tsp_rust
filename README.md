# Traveling Salesman Problem Solver in Rust

A zero-dependency TSP solver with three distinct implementations showcasing algorithmic optimizations and parallel computing in Rust.

## Features

- **Three Implementations**: Single-threaded, multi-threaded, and optimized with advanced algorithms
- **Exact Solutions**: All implementations find the mathematically optimal solution (not heuristic)
- **Performance Benchmarking**: Randomized execution order and detailed performance comparisons
- **Scalable**: Handles 1-20+ cities with different algorithmic approaches based on problem size

## Algorithms

| Implementation | Approach | Best For |
|----------------|----------|----------|
| **Single-threaded** | Brute force permutation | Baseline comparison, 1-10 cities |
| **Multi-threaded** | Parallel brute force | CPU-bound improvement, 4-12 cities |
| **Optimized** | Distance matrix + Branch & Bound + Bitmask DP | Performance critical, 1-20+ cities |

## Performance

The optimized implementation demonstrates dramatic improvements through algorithmic optimization:

```bash
# 14 cities benchmark results:
1. Optimized:      0.008s  (baseline)
2. Multi-threaded: 59.084s (7,580x slower)  
3. Single-threaded: 432.124s (55,444x slower)
```

**Key optimizations:**
- Pre-computed distance matrix (eliminates repeated sqrt calculations)
- Branch-and-bound pruning (reduces search space exponentially)
- Bitmask dynamic programming (O(n²2ⁿ) vs O(n!) for ≤20 cities)

## Usage

```bash
cargo run --release <num_cities> [seed] [threads]
```

**Examples:**
```bash
cargo run --release 10 42        # 10 cities, seed 42, auto-detect threads
cargo run --release 15 123 8     # 15 cities, seed 123, 8 threads
```

**Recommendations:**
- **1-10 cities**: All implementations work well
- **11-15 cities**: Use optimized implementation for best performance
- **16-20 cities**: Only optimized implementation practical
- **20+ cities**: Consider heuristic algorithms for larger problems

## Sample Output

```
=== Traveling Salesman Problem Solver ===
Cities: 13, Seed: 42, Available CPU threads: 16

=== Performance Summary ===
Implementation order: ["Multi", "Optimized", "Single"]

Performance Ranking:
  1. Optimized: 0.004s (distance: 403.93)
  2. Multi-threaded: 4.400s (distance: 403.93, 1,243x slower)
  3. Single-threaded: 32.522s (distance: 403.93, 9,190x slower)

✓ All implementations found the same optimal solution!
```

## Building & Testing

```bash
# Build optimized binary
cargo build --release

# Run tests
cargo test

# Run with custom parameters
./target/release/tsp_solver 12 42 4
```

## Technical Details

- **Language**: Rust (zero external dependencies)
- **Algorithms**: Exact solutions only - guaranteed optimal results
- **Threading**: Work-stealing parallel permutation generation
- **Memory**: Efficient distance matrix caching and memory pooling
- **Verification**: All implementations cross-validated for correctness

## Complexity Analysis

| Cities | Brute Force | Optimized (Bitmask DP) | Practical Runtime |
|--------|-------------|------------------------|-------------------|
| 10 | 10! ≈ 3.6M | 10²×2¹⁰ ≈ 102K | milliseconds |
| 15 | 15! ≈ 1.3T | 15²×2¹⁵ ≈ 7.4M | seconds |
| 20 | 20! ≈ 2.4×10¹⁸ | 20²×2²⁰ ≈ 419M | minutes |

**⚠️ AI Disclosure**: This project includes code generated with assistance from Large Language Models (LLMs). 

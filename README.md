# Parallel quicksort

A quick port of https://easyprog.app/blog/sort_par.html to Rust


```
-% go run sort.go
Sorting 50 million numbers with Goroutines in Go ...
Time: 1.203s

-% cargo run --release
    Finished release [optimized] target(s) in 0.09s
     Running `target/release/parqsort`
Sorting 50 million numbers with naive quicksort in Rust ...
Time: 8.36s
Sorting 50 million numbers with stdlib quicksort in Rust ...
Time: 4.19s
Sorting 50 million numbers with naive parallel quicksort in Rust ...
Time: 1.16s
Sorting 50 million numbers with Rayon parallel quicksort in Rust ...
Time: 528.86ms
```


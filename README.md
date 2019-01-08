# Parallel quicksort

A quick port of https://easyprog.app/blog/sort_par.html


```
-% go run sort.go
Sorting 50 million numbers with Goroutines in Go ...
Time: 1.203s

-% cargo run --release
    Finished release [optimized] target(s) in 0.09s
     Running `target/release/parqsort`
Sorting 50 million numbers with Quicksort in Rust
Time: 1.13s
Sorting 50 million numbers with Rayon in Rust
Time: 495.70ms
```


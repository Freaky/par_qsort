use rayon::join;

fn partition<T, F>(d: &mut [T], is_less: &F) -> usize
where
    F: Fn(&T, &T) -> bool,
{
    d.swap(0, d.len() / 2);
    let mut mid = 0;
    for i in 1..d.len() {
        if is_less(&d[i], &d[0]) {
            mid += 1;
            d.swap(i, mid);
        }
    }
    d.swap(0, mid);
    mid
}

fn insert_sort<T, F>(d: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool,
{
    for i in 1..d.len() {
        let mut n = i;
        while n > 0 && is_less(&d[n], &d[n - 1]) {
            d.swap(n, n - 1);
            n -= 1;
        }
    }
}

fn quick_sort<T, F>(d: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool,
{
    if d.len() > 30 {
        let mut mid = partition(d, is_less);
        if mid < d.len() / 2 {
            mid += 1;
        }
        let (left, right) = d.split_at_mut(mid);
        quick_sort(left, is_less);
        quick_sort(right, is_less);
    } else {
        insert_sort(d, is_less);
    }
}

fn par_quick_sort<T, F>(d: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool + Send + Sync,
    T: Send,
{
    if d.len() > 30 {
        let mut mid = partition(d, is_less);
        if mid < d.len() / 2 {
            mid += 1;
        }
        let (left, right) = d.split_at_mut(mid);

        if right.len() > 100_000 {
            join(
                || par_quick_sort(left, is_less),
                || par_quick_sort(right, is_less),
            );
        } else {
            quick_sort(left, is_less);
            quick_sort(right, is_less);
        }
    } else {
        insert_sort(d, is_less);
    }
}

pub trait QSort<T> {
    fn qsort(&mut self);
}

impl<T> QSort<T> for [T]
where
    T: Ord,
{
    fn qsort(&mut self) {
        quick_sort(self, &|a: &T, b: &T| a.lt(b));
    }
}

pub trait ParQSort<T> {
    fn par_qsort(&mut self);
}

impl<T> ParQSort<T> for [T]
where
    T: Ord + Send,
{
    fn par_qsort(&mut self) {
        par_quick_sort(self, &|a: &T, b: &T| a.lt(b));
    }
}

use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let len = 50_000_000;
    let mut orig: Vec<u32> = vec![0; len];
    thread_rng().fill(&mut orig[..]);

    let mut data = orig.clone();
    let start = Instant::now();
    println!(
        "Sorting {} million numbers with naive quicksort in Rust ...",
        len / 1_000_000
    );
    data.qsort();
    println!("Time: {:.2?}", start.elapsed());

    let mut data = orig.clone();
    let start = Instant::now();
    println!(
        "Sorting {} million numbers with stdlib quicksort in Rust ...",
        len / 1_000_000
    );
    data.sort_unstable();
    println!("Time: {:.2?}", start.elapsed());

    let mut data = orig.clone();
    let start = Instant::now();
    println!(
        "Sorting {} million numbers with naive parallel quicksort in Rust ...",
        len / 1_000_000
    );
    data.par_qsort();
    println!("Time: {:.2?}", start.elapsed());

    let mut data = orig.clone();
    let start = Instant::now();
    println!(
        "Sorting {} million numbers with Rayon parallel quicksort in Rust ...",
        len / 1_000_000
    );
    data.par_sort_unstable();
    println!("Time: {:.2?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn test_all(d: Vec<i32>) -> bool {
            let mut d = d;

            let mut expected = d.clone();
            expected.sort_unstable();
            d.par_qsort();

            expected == d
        }
    }
}

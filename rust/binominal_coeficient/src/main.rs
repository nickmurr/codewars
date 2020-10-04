// use std::iter;
#![allow(unused_variables)]
#![allow(dead_code)]

pub struct PascalsTriangle {
    row_count: usize,
    index: usize,
}

impl PascalsTriangle {
    pub fn new(row_count: usize, index: usize) -> Self {
        PascalsTriangle { row_count: row_count + 1, index }
    }

    pub fn rows(&self) -> Vec<Vec<usize>> {
        let mut tri = vec![vec![]; self.row_count];
        for i in 0..self.row_count {
            let mut row = vec![1; i + 1];
            let mut val = 1;

            for j in 0..i + 1 {
                if j > self.index { continue; }
                row[j] = val;
                val = val * (i - j) / (j + 1);
            }
            tri[i] = row;
        }

        tri
    }
}

fn main() {
    let res = diagonal2(7, 2);
    println!("{}", res);
}

fn diagonal(n: u32, p: u32) -> u64 {
    let row: usize = n as usize;
    let index: usize = p as usize;

    if index == 0 {
        return (n + 1) as u64;
    }

    let x = PascalsTriangle::new(row, index);
    let mut sum: usize = 0;
    for (idx, val) in x.rows().iter().enumerate() {
        if idx >= index {
            sum += val[index];
        }
    }

    sum as u64
}

fn diagonal2(n: u32, p: u32) -> u64 {
    let b = (1..=(p + 1)).fold(1.0, |f, i| {
        let v = f * (i as f64);
        println!("a: f = {}, i = {}, v = {}", f, i, v);
        v
    });
    println!();
    let a = ((n - p + 1)..=(n + 1)).fold(1.0 / b, |f, i| {
        let v = f * (i as f64);
        println!("b: f = {}, i = {}, v = {}", f, i, v);
        v
    });
    println!();
    a.round() as u64
}


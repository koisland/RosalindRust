use std::{fs};
use itertools::Itertools;


pub fn longest_subseq(rs_fname: &str, ascending: bool) -> String {
    let contents: Vec<String> = fs::read_to_string(rs_fname)
        .expect("Unable to read file")
        .trim()
        .replace("\r", "")
        .split("\n")
        .map(|res| res.to_string())
        .collect();
    
    let (_, seq) = (
        &contents[0].parse::<usize>().expect("Unable to parse to usize"),
        &contents[1].split(" ").map(|num| num.parse::<usize>().expect("Unable to parse usize in seq.")).collect::<Vec<usize>>()
    );
    
    let mut M: Vec<usize> = vec![0; seq.len()];
    let mut P: Vec<usize> = vec![0; seq.len()];

    let mut L = 1;
    M[0] = 0;
    // Iterate thru each element in seq.
    for idx in 1..seq.len() {
        let mut lower = 0;
        let mut upper = L;
        
        let mut j: usize;
        let upper_bnd_check = match ascending {
            true =>seq[M[upper - 1]].lt(&seq[idx]),
            false => seq[M[upper - 1]].gt(&seq[idx])
        };

        if upper_bnd_check {
            j = upper;
        } else {
            while upper - lower > 1 {
                let mid = (upper + lower) / 2;
                let bin_search_check = match ascending {
                    true =>seq[M[mid-1]].lt(&seq[idx]),
                    false => seq[M[mid-1]].gt(&seq[idx])
                };
                if bin_search_check {
                    lower = mid;
                } else {
                    upper = mid;
                }
            }

            j = lower;
        }
        // Handle not being able to access elements using i32.
        if j == 0 {
            j = M.len();
            P[idx] = M[j - 1];
            j = 0;
        } else {
            P[idx] = M[j - 1];
        }
        

        let final_check = match ascending {
            true =>seq[idx].lt(&seq[M[j]]),
            false => seq[idx].gt(&seq[M[j]])
        };
        if j == L || final_check {
            M[j] = idx;
            L = *vec![L, j + 1].iter().max().unwrap();
        }

    }

    let mut result: Vec<usize> = vec![];
    let mut pos = M[L-1];
    for _ in 0..L {
        result.push(seq[pos]);
        pos = P[pos];
    }
    let str_result = result.into_iter().rev().join(" ");

    str_result
}
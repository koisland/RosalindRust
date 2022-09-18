use std::{fs, usize, collections::HashMap};
use itertools::{Itertools, iproduct};

pub fn idx_str(string: &str, idx_tbl: &HashMap<char, usize>) -> String {
    let ord_string = string
        .chars()
        .map(|chr| idx_tbl.get(&chr).unwrap_or(&0))
        // Pad idx as string to 2 digits.
        .map(|idx| format!("{:0>2}", idx))
        .join("");
    ord_string
}

pub fn lex_order(rs_fname: &str) -> String {
    let contents: Vec<String> = fs::read_to_string(rs_fname)
        .expect("Unable to read file")
        .trim()
        .replace("\r", "")
        .split("\n")
        .map(|res| res.to_string())
        .collect();

    let (alpha, num) = (&contents[0], &contents[1].parse::<usize>().expect("Unable to parse to usize"));
    
    let mut alpha: Vec<char> = alpha
        .split(" ")
        .map(|alpha_chr| alpha_chr.chars().nth(0).expect("No char."))
        .collect();

    // Add spacer char.
    alpha.extend(vec!['0']);

    let mut all_str_prod: Vec<Vec<char>> = vec![vec![]];

    // Repeat cartesian product for as many characters the longest string will be.
    for _ in 0..*num {
        all_str_prod = iproduct!(all_str_prod.iter(), alpha.iter())
            .map(|(v, x)| {
                let mut v1 = v.clone(); 
                let x1 = x.clone();
                v1.push(x1);
                v1
            })
            .collect_vec();
    }

    // Join character vectors to make strings.
    // Remove 0 placeholders.
    let mut all_str: Vec<String> = all_str_prod
        .iter()
        .map(|str_chars| str_chars.into_iter().join("").replace("0", ""))
        .collect_vec();

    // Generate index table to lexicographically sort strings using our defined alphabet.
    let idx_tbl: HashMap<char, usize> = alpha
        .iter()
        .enumerate()
        .map(|(ord, chr)| (*chr, ord))
        .collect();

    // Sort by strings generated from our index table.
    all_str.sort_by(|a, b| {
        let a = idx_str(&a, &idx_tbl);
        let b = idx_str(&b, &idx_tbl);
        a.cmp(&b)
    });
    // Remove duplicates. Must be done after sort as only adjacent values will be culled.
    all_str.dedup();

    let agg_str_res = all_str.join("\n").trim().to_string();
    agg_str_res
}
use crate::lib::rs_utils::{hamm_dst, unpack_fasta};
use ndarray::Array;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn distance_matrix(rs_fname: &str) -> String {
    // Can't use unpack as loads into unordered hashmap.
    // NOTE: Will panic if seq split over multiple lines.
    // let seqs = unpack_fasta(rs_fname);
    let seqs: Vec<String> = read_to_string(rs_fname)
        .expect("Unable to read file")
        .split("\r\n")
        .filter(|fileline| !fileline.starts_with('>') && !fileline.is_empty())
        .map(|seq| seq.to_string())
        .collect();

    let mut dst_matrix: Array<f64,_> = Array::zeros((seqs.len(), seqs.len()));
    let seq_dim_map: HashMap<String, usize> = seqs
        .iter()
        .enumerate()
        .map(|(i, seq)| (seq.to_owned(), i))
        .collect();

    // Generate all permutations. 
    for perm in seqs.iter().permutations(2) {
        if let [seq_1, seq_2] = perm[..] {
            let dst = hamm_dst(seq_1, seq_2) as f64 / seq_1.len() as f64;
            // println!("Distance between {}-{}: {}", seq_1, seq_2, dst);

            let row = seq_dim_map.get(seq_1).unwrap_or_else(|| panic!("Seq {} doesn't exist.", seq_1));
            let col = seq_dim_map.get(seq_2).unwrap_or_else(|| panic!("Seq {} doesn't exist.", seq_2));
            dst_matrix[[*row, *col]] = dst;
        }
    }
    let mut dst_matrix_str = String::new();
    let dst_matrix_shape = dst_matrix.shape();

    for row in 0..dst_matrix_shape[0] {
        let mut col_vals = (0..dst_matrix_shape[1])
            .into_iter()
            .map(|col| &dst_matrix[[row, col]])
            .join(" ");
        // Need to add space to replace 0 with 0.0
        col_vals.push_str(" ");

        dst_matrix_str.push_str(col_vals.replace("0 ", "0.0 ").trim());
        dst_matrix_str.push('\n');
    }

    dst_matrix_str
}

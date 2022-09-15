use std::fs;
use std::collections::HashMap;
use itertools::{Itertools, iproduct};

use crate::lib::rs_utils::unpack_fasta;


fn gen_possible_4_kmers(nts: Vec<char>) -> Vec<String> {
    let (kmer_nt_1, kmer_nt_2, kmer_nt_3, kmer_nt_4) = 
        (nts.clone(), nts.clone(), nts.clone(), nts.clone());
    let possible_kmers = iproduct!(kmer_nt_1, kmer_nt_2, kmer_nt_3, kmer_nt_4)
        .map(|nts| vec![nts.0, nts.1, nts.2, nts.3].iter().join(""))
        .collect();
    possible_kmers
}

pub fn kmer_comp(fname: &String, kmer_size: usize) -> Vec<usize> {
    let contents = unpack_fasta(fname);
    let mut n_kmers:HashMap<&str, usize> = HashMap::new();

    // Trim to exclude newline from being sliced.
    for (_, content) in contents.iter() {
        let content_len = content.trim().len();
        for i in 0..content_len {
            // End of string.
            if i + kmer_size > content_len {
                break;
            }

            let kmer = &content[i..i + kmer_size];
            if kmer.len() != kmer_size {
                break;
            }

            // Insert new kmer and continue to next if successful.
            if !n_kmers.contains_key(kmer) {
                n_kmers.insert(kmer, 1);
                continue;
            }
            if let Some(kmer_cnt) = n_kmers.get_mut(kmer) {
                *kmer_cnt += 1
            }
        }
    }

    let possible_kmers: Vec<String> = gen_possible_4_kmers(
        vec!['A', 'T', 'G', 'C']
    );
    let mut all_kmers = n_kmers
        .keys()
        .cloned()
        .collect::<Vec<&str>>()
        .iter()
        .map(|kmer| kmer.to_string())
        .collect::<Vec<String>>();

    for kmer in possible_kmers.iter() {
        if !all_kmers.contains(kmer) {
            all_kmers.push(kmer.to_string());
        }
    }
    all_kmers.sort();

    let mut all_kmer_comp: Vec<usize> = Vec::new();
    for kmer in all_kmers.iter() {
        if let Some(poss_kmer) = n_kmers.get(&kmer[..]) {
            all_kmer_comp.push(poss_kmer.to_owned());
        } else {
            all_kmer_comp.push(0);
        }
        
    }
    all_kmer_comp
}
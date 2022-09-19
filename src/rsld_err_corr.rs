use itertools::Itertools;
use std::collections::HashMap;
use crate::lib::rs_utils::{unpack_fasta, rev_comp, hamm_dst};


pub fn error_correction(fname: &str) -> String {
    let seqs = unpack_fasta(&fname);
    let mut seq_cnts: HashMap<String, usize> = HashMap::new();

    for (_, seq) in seqs.iter() {
        let rev_seq: String = rev_comp(&seq);
        // println!("{} {} {}", id, seq, rev_seq);

        if !seq_cnts.contains_key(seq) {
            if seq_cnts.contains_key(&rev_seq) {
                if let Some(seq_cnt)= seq_cnts.get_mut(&rev_seq) {
                    *seq_cnt += 1
                } 
            } else {
                seq_cnts.insert(seq.to_owned(), 1);
            }
            
        } else {
            if let Some(seq_cnt)= seq_cnts.get_mut(seq) {
                *seq_cnt += 1
            } 
        }
    }

    let mut seq_errs: Vec<String> = Vec::new();
    for (seq, cnt) in seq_cnts.iter() {
        
        if *cnt == 1 {
            let rev_seq = rev_comp(seq);
            // println!("Checking hamming distance for {} & {}", seq, rev_seq);
            for (comp_seq, _) in seq_cnts.iter() {
                if comp_seq == seq {
                    continue;
                }
                let h_dist = hamm_dst(seq, &comp_seq);
                let h_dist_rev = hamm_dst(&rev_seq, &comp_seq);

                // Output format is [old]->[new]
                if h_dist == 1 {
                    seq_errs.push(format!("{}->{}", seq, comp_seq));
                } 
                if h_dist_rev == 1 {
                    let rev_comp_seq = rev_comp(comp_seq);
                    seq_errs.push(format!("{}->{}", seq, rev_comp_seq));
                }
            }
        }
    }
    seq_errs.join("\n")
}

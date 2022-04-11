use std::collections::HashMap;
use crate::lib::rs_utils::unpack_fasta;

pub fn locate_rsites(fname: &String) -> Vec<(usize, usize)> {
    let seqs = unpack_fasta(fname);
    let nt_comp = HashMap::from([
        ('A', 'T'),
        ('T', 'A'),
        ('G', 'C'),
        ('C', 'G')
    ]);
    
    let mut rsites: Vec<(usize, usize)> = vec![];

    for (_, seq) in seqs.iter() {
        let seq_len = seq.chars().count();

        for (i, _) in seq.chars().enumerate() {
            for j in 1..6 {
                let upper_idx = i + j;

                if (j > i) | (upper_idx > seq_len) {
                    break;
                }

                let forw_nts = &seq[i..i+j].chars()
                    .rev()
                    .map(|chr| nt_comp.get(&chr).expect("Nt doesn't exist."))
                    .collect::<String>();
                let prev_nts = &seq[i-j..i].to_string();


                if forw_nts.eq(prev_nts) {
                    // println!("seq: {}\nr:{} f:{}\ni:{} len: {}", &seq[i-j..i+j], prev_nts, forw_nts, i-j+1, j * 2);
                    // println!("Equals: {}\n", forw_nts.eq(prev_nts));
                    let rsite_len = j * 2;
                    if rsite_len >= 4 {
                        rsites.push((i-j+1, rsite_len));
                    }    
                }


            }

            
        }
    }

    return rsites;
}

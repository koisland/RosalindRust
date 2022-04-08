use std::collections::HashMap;
use crate::lib::rs_utils::unpack_fasta;

pub struct HighestGC {
    pub id: String,
    pub gc_perc: f64
}

pub fn gc_content(fname: &String) -> HighestGC {
    
    let mut res = HighestGC {id: "".to_string(), gc_perc: 0.0};

    let seqs: HashMap<String, String> = unpack_fasta(fname);

    for (seq_id, seq) in seqs.iter() {
        let mut gc_counter: u64 = 0;

        let seq_len = seq.len() as f64;

        for nt in seq.chars() {
            match &nt {
                'G' | 'C' => gc_counter += 1,
                _ => ()
            }
        }

        let gc_perc: f64 = gc_counter as f64 / seq_len;

        // if gc_perc higher, replace res gc_perc and id
        if res.gc_perc < gc_perc {
            res.id = seq_id.to_string();
            res.gc_perc = gc_perc;
        }
    }

    return res
}
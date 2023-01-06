use crate::lib::rs_utils::unpack_fasta;
use itertools::Itertools;

fn factorial(n: usize) -> u128 {
    let mut prod: u128 = 1;
    for i in 0..n {
        let i = i as u128;
        prod *= i + 1
    }

    return prod;
}

pub fn pmch(fname: &str) -> String {
    let seqs = unpack_fasta(fname);
    for (_, seq) in seqs.iter() {
        let num_nodes = seq.len();
        println!("Nodes: {}", num_nodes);
        println!("{}", seq);

        let nt_cnt = seq.chars().counts();
        println!("{:?}", nt_cnt);

        let num_at_bonds = if let Some(num_at) = nt_cnt.get(&'A') {
            factorial(*num_at)
        } else {
            0
        };
        let num_gc_bonds = if let Some(num_gc) = nt_cnt.get(&'G') {
            factorial(*num_gc)
        } else {
            0
        };

        let n_pmch = num_at_bonds * num_gc_bonds;
        println!("{n_pmch}")
    }
    "".to_string()
}

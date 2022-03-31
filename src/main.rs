
use std::env;

// mod rsld_dna;
// use crate::rsld_dna::rsld_dna::cnt_nt;

mod rsld_rna;
use crate::rsld_rna::rs_rna::replace_u;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rs_fname = &args[1];
    
    // cnt_nt(rs_fname);
    replace_u(rs_fname);
}

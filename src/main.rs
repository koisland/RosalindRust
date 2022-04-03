use std::env;

// mod rsld_dna;
// use crate::rsld_dna::rsld_dna::cnt_nt;

// mod rsld_rna;
// use crate::rsld_rna::rs_rna::replace_u;

// mod rsld_revc;
// use crate::rsld_revc::rs_revc::rev_comp;

// mod rsld_hamm;
// use crate::rsld_hamm::rs_hamm::hamm_dst;

mod rsld_subs;
use crate::rsld_subs::rs_subs::find_motif;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rs_fname = &args[1];
    
    // let res = cnt_nt(rs_fname);
    // let res = replace_u(rs_fname);
    // let res = rev_comp(rs_fname);
    // let res = hamm_dst(rs_fname);
    let res = find_motif(rs_fname);

    println!("{}", res);

}

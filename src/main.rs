use std::env;

mod lib;

// mod rsld_dna;
// use crate::rsld_dna::cnt_nt;

// mod rsld_rna;
// use crate::rsld_rna::replace_u;

// mod rsld_revc;
// use crate::rsld_revc::rev_comp;

// mod rsld_hamm;
// use crate::rsld_hamm::hamm_dst;

// mod rsld_subs;
// use crate::rsld_subs::find_motif;

// mod rsld_prot;
// use crate::rsld_prot::translate;

// mod rsld_gc;
// use crate::rsld_gc::gc_content;

mod rsld_revp;
use crate::rsld_revp::locate_rsites;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rs_fname = &args[1];
    
    // let res = cnt_nt(rs_fname);
    // let res = replace_u(rs_fname);
    // let res = rev_comp(rs_fname);
    // let res = hamm_dst(rs_fname);
    // let res = find_motif(rs_fname);
    // let res = translate(rs_fname);
    // let res = gc_content(rs_fname);

    let res = locate_rsites(rs_fname);
    println!("{:#?}", res);

}

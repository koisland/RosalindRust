use itertools::Itertools;
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

// mod rsld_kmer_comp;
// use crate::rsld_kmer_comp::kmer_comp;

// mod rsld_enum;
// use crate::rsld_enum::signed_perm;

// mod rsld_lexv;
// use crate::rsld_lexv::lex_order;

// mod rsld_longest_subs;
// use crate::rsld_longest_subs::longest_subseq;

mod rsld_dst_mat;
use crate::rsld_dst_mat::distance_matrix;

// // TODO:?
// mod rsld_err_corr;
// use crate::rsld_err_corr::error_correction;

// TODO:?
// mod rsld_revp;
// use crate::rsld_revp::locate_rsites;

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
    // let res = locate_rsites(rs_fname);
    // let res = kmer_comp(rs_fname, 4).iter().join(" ");
    // let res = signed_perm(&rs_fname[..]);
    // let res = lex_order(rs_fname);
    ////
    // let res = longest_subseq(rs_fname, true);
    // let desc_res = longest_subseq(rs_fname, false);

    // TODO
    // let res = error_correction(&rs_fname[..]);
    let res = distance_matrix(rs_fname);
    println!("{}", res);
}

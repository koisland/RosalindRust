mod rsld_dna;

use std::env;
use crate::rsld_dna::rs_dna::cnt_nt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rs_fname = &args[1];
    cnt_nt(rs_fname);
}

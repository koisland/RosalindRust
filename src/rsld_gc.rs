use std::fs;
use crate::lib::rs_utils::unpack_fasta;

pub struct HighestGC {
    pub id: String,
    pub gc_perc: f32
}

pub fn gc_content(fname: &String) -> HighestGC {
    
    let res = HighestGC {id: "".to_string(), gc_perc: 0.0};

    return res
}
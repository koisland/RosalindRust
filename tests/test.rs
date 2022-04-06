
use rosalind_rust::rs_utils::*;

#[cfg(test)]
mod tests {
    use rosalind_rust::rs_utils::{unpack_fasta, rna_codons};

    #[test]
    fn test_unpack() {
        unpack_fasta();
    }

    fn test_rna_codons() {
        rna_codons();
    }
}
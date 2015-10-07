
pub fn generate_three_mer(input: &str) -> [usize; 64] {
    let mut kmer = [0; 64];
    let bytes = input.as_bytes();
    let mut a = bytes[0]   & 0b110;
    let mut b = bytes[1] & 0b110;
    for i in 2..bytes.len() {
        let c = bytes[i] & 0b110;

        let k_index : usize = ((a << 3) | (b << 1) | (c >> 1)) as usize;
        kmer[k_index] += 1;

        a = b;
        b = c;
    }
    kmer
}

pub fn hellinger_distance(kmer1: &[usize; 64], kmer2: &[usize; 64]) -> f32 {
    let mut hellinger = 0.0;
    for i in 0..kmer1.len() {
        let a = kmer1[i];
        let b = kmer2[i];
        hellinger += a as f32 + b as f32 - 2f32 * ((a * b) as f32).sqrt();
    }
    hellinger
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_three_mer() {
        let dna1 = "aaaagaaa";
        let dna2 = "aaagaaaa";

        let kmers1 = generate_three_mer(dna1);
        let kmers2 = generate_three_mer(dna2);

        for i in 0..kmers1.len() {
            assert_eq!(kmers1[i], kmers2[i]);
        }
    }

    #[test]
    fn test_hellinger_distance() {
        let dna1 = "aaacgtttggcagatcgcgtactgactactactgactgtacgtagtcgcgaacgtcga";
        let dna2 = "tgcatgccctatcggaattgccattttggcatcgagcatgcatcagggcatcagctac";
        let kmer1 = generate_three_mer(dna1);
        let kmer2 = generate_three_mer(dna2);

        // Distance should be reflexive
        assert_eq!(0f32, hellinger_distance(&kmer1, &kmer1));
        // Distance should be symmetrical
        assert_eq!(hellinger_distance(&kmer1, &kmer2), hellinger_distance(&kmer2, &kmer1));
    }
}

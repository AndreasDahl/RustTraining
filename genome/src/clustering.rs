use std::io::Write;
use std::collections::LinkedList;
use std::f32::INFINITY;

use kmer::kmer;
use fasta_string::FastaString;

pub fn seq_clust(input: &mut Iterator<Item = FastaString>, similarity: f32, output: &mut Write) {
    let mut seeds : LinkedList<FastaString> = LinkedList::new();
    let mut cluster_count = 0;
    for current in input {
        let size = current.sequence.len();
        // Generate kmer
        let current_kmer = kmer::generate_three_mer(&current.sequence);

        let mut best_dist = INFINITY;
        let mut n_from_last = 0;
        for seed in seeds.iter_mut() {
            let seed_kmer = kmer::generate_three_mer(&seed.sequence);
            best_dist = kmer::hellinger_distance(&current_kmer, &seed_kmer);
            if best_dist < similarity {
                writeln!(output,
                         "H\t{cluster}\t{size}\t{id}\t+\t0\t0\t???\t{query}\t{target}",
                         cluster = cluster_count - n_from_last,
                         size = size,
                         id = best_dist,
                         query = &current.description,
                         target = seed.description).ok();
                break;
            }
            n_from_last += 1;
        }
        if best_dist > similarity {
            cluster_count += 1;
            if seeds.len() >= 32 {
                seeds.pop_back();
            }
            writeln!(output, "S\t{}\t{}\t*\t*\t*\t*\t*\t{}\t*", cluster_count, size,
                     current.description).ok();
            seeds.push_front(current);
        } else {
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use std::io;
    use std::fs;

    use fasta::FastaIO;
    use super::seq_clust;

    #[test]
    fn test_seq_clust() {
        let mut fasta = FastaIO::new(fs::File::open("res/p3_clean_C-148-2-Caecum_S128_sorted.fa").unwrap());

        seq_clust(&mut fasta, 0.5, &mut io::sink());
    }

    // Benches ----

}

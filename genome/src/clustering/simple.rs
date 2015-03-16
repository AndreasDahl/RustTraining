use std::io::Write;
use std::collections::LinkedList;
use std::num::Float;

use kmer::kmer;

pub fn seq_clust(input: &mut Iterator<Item = &str>, similarity: f32, output: &mut Write) {
    let mut seeds = LinkedList::new();
    let mut cluster_count = 0;
    for current in input {
        let size = current.len();
        // Generate kmer
        let current_kmer = kmer::generate_three_mer(current);

        let mut best_dist = Float::infinity();
        let mut n_from_last = 0;
        for seed in &seeds {
            best_dist = kmer::hellinger_distance(&current_kmer, seed);
            if best_dist < similarity {
                break;
            }
            n_from_last += 1;
        }
        if best_dist > similarity {
            cluster_count += 1;
            if seeds.len() >= 32 {
                seeds.pop_back();
            }
            seeds.push_front(current_kmer);
            writeln!(output, "S\t{}\t{}\t*\t*\t*\t*\t*\t{}\t*", cluster_count, size, "TEST");
        } else {
            writeln!(output,
                     "H\t{cluster}\t{size}\t{id}\t+\t0\t0\t???\t{query}\t{target}", 
                     cluster = cluster_count - n_from_last, 
                     size = size,
                     id = best_dist,
                     query = "TEST",
                     target = "TEST");
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use std::io;

    use super::seq_clust;
	
    #[test]
    fn test_seq_clust() {
        let data = vec!["abc", "abc", "abd", "aabc", "cbd", "abc"];
        //let mut out = io::Cursor::new(Vec::new());
        
        seq_clust(&mut data.into_iter(), 0.5, &mut io::stdout());
        
        //let s = String::from_utf8(out.into_inner()).ok().expect("ERROR");

    }
}

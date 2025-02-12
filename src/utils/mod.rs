mod blosum;
mod count;
mod dna;
mod fasta;
pub mod hamming;
mod kmp;
mod nucleotide;
mod reverse;
mod rna;
mod time;
mod transpose;
mod union;
mod vec;

pub use blosum::blosum_matrix;
pub use dna::{DNA, DNA_INDEX};
pub use fasta::Fasta;
pub use nucleotide::{InvalidNucleotideError, InvalidNucleotidePositionError};
pub use reverse::dna_complement;
pub use rna::dna_to_rna;
use std::collections::HashMap;
pub use time::print_hms;
pub use transpose::transpose;
pub use union::{find_parent, union};
pub use vec::vec_to_count;

pub type Graph<T> = HashMap<T, Vec<T>>;
pub type WeightedGraph<T, S> = HashMap<T, Vec<(T, S)>>;

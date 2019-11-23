use {Matrix, Dict};
use std::io::prelude::*;
use std::fs::File;

use utils::W2vError;
pub struct Word2vec {
    syn0: Matrix,
    syn1neg: Matrix,
    dim: usize,
    dict: Dict,
}

impl Word2vec {
    pub fn new(syn0: Matrix, syn1neg: Matrix, dim: usize, dict: Dict) -> Word2vec {
        Word2vec {
            syn0: syn0,
            syn1neg: syn1neg,
            dim: dim,
            dict: dict,
        }
    }
    pub fn norm_self(&mut self) {
        self.syn0.norm_self();
    }
    #[cfg(feature="blas")]
    #[inline(always)]
    pub fn most_similar(&self, word: &str, topn: Option<usize>) -> Vec<(f32, String)> {
        let mut vec = vec![0.;self.dict.nsize()];
        let c = self.dict.get_idx(word);
        let row = self.syn0.get_row_unmod(c);
        self.syn0.sgemv(row, vec.as_mut_ptr());
        let mut sorted = Vec::new();
        for i in 0..vec.len() {
            sorted.push((vec[i], self.dict.get_word(i)));
        }
        sorted.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        let topn = topn.unwrap_or(10);
        sorted.into_iter().take(topn).collect()
    }
    
    pub fn save_vectors(&self, filename: &str) -> Result<bool, W2vError> {
        let size = self.dict.nsize();
        let mut file = try!(File::create(filename));
        let mut meta = Vec::new();

        try!(write!(&mut meta, "{} {}\n", size, self.dim));
        try!(file.write_all(&meta));
        let start = self.syn0.get_row_unmod(0);
        for i in 0..size {
            try!(file.write_all(&self.dict.get_word(i).into_bytes()[..]));
            for j in 0..self.dim {
                unsafe {
                    let s = format!(" {}", *start.offset((i * self.dim + j) as isize));
                    try!(file.write(&s.into_bytes()[..]));
                }
            }
            try!(file.write(b"\n"));
        }
        Ok(true)
    }

}

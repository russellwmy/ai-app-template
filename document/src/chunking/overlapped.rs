use rayon::prelude::*;
use tiktoken_rs::{p50k_base, CoreBPE};

pub struct OverlappedChunker {
    token_counter: CoreBPE,
    max_token: usize,
}

impl OverlappedChunker {
    pub fn new() -> Self {
        Self {
            token_counter: p50k_base().unwrap(),
            max_token: 500,
        }
    }
    pub fn with_size(size: usize) -> Self {
        Self {
            token_counter: p50k_base().unwrap(),
            max_token: size,
        }
    }
}

impl OverlappedChunker {
    pub fn chunks(&self, lines: &Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut buf = String::new();

        let chunk_token = self.max_token / 2;
        for line in lines {
            let buf_token_count = self.token_counter.encode_with_special_tokens(&buf).len();
            let line_token_count = self.token_counter.encode_with_special_tokens(&line).len();

            if (line_token_count + buf_token_count) > chunk_token {
                result.push(buf);
                buf = String::new();
            }
            buf.push_str(&line);
            buf.push_str("\n");
        }
        result
            .windows(2)
            .map(|w| {
                let mut chunk = String::from(w[0].to_string());

                chunk.push_str(&w[1]);
                chunk
            })
            .collect::<Vec<String>>()
    }
}

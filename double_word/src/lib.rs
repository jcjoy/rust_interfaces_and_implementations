//! double_word
//! Rust library performing implementing the utilities to count double words from an input stream
//! 
//! Author: James C. Joy

use std::io::Read;
use std::io::Write;
use std::io::BufRead;

pub fn double_word<T: Read, U: Write>(reader: &mut T, writer: &mut U ) -> Result<(), Box<dyn std::error::Error>>{
    //double_word Read inputs and print out doubled words and lines
    
    let mut prev_word = String::new();
    let buf_read = std::io::BufReader::new(reader);
    for (idx, line) in buf_read.lines().enumerate(){
        let words = line.unwrap();
        let word_iter = words.split_whitespace();
        for word in word_iter {
            if prev_word.eq(word){
               write!(writer, "{}: {}\n", idx, word);
            }
            prev_word = word.to_string();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works()  -> Result<(),Box<dyn std::error::Error>>{
       double_word(&mut std::io::stdin(), &mut std::io::stdout())?;
        Ok(())
    }
}

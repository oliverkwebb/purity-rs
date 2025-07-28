use std::fs::File;
#[warn(missing_docs)]
use std::io::prelude::*;

/// A purity file is a sequence of blocks.
enum PurityBlock {
    PlainText(String),
    SubjectHeader(String, usize),
    Question(String, usize),
    Conclusion(String),
}

struct PurityParser<R: Read> {
    subject_number: usize,
    question_number: usize,
    byte_source: R,
}

impl<R: Read> PurityParser<R> {
    pub fn new(input: R) -> Self {
        PurityParser {
            subject_number: 0,
            question_number: 0,
            byte_source: input,
        }
    }
}

impl<R: Read> Iterator for PurityParser<R> {
    type Item = PurityBlock;

    fn next(&mut self) -> Option<Self::Item> {
        // All out-of-block characters are treated as comments
        // TODO: For Now, Read the file as a block
        if let Ok(s) = std::io::read_to_string(&mut self.byte_source) {
            if s.len() > 0 {
                return Some(PurityBlock::PlainText(s));
            } else {
                return None;
            }
        } else {
            None
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = File::open("hacker")?;
    let test: Vec<PurityBlock> = PurityParser::new(input).collect();
    for block in test {
        match block {
            PurityBlock::PlainText(header) => println!("{}", header),
            PurityBlock::SubjectHeader(s, n) => println!("---\nSection {}. {}", n, s),
            PurityBlock::Question(s, n) => println!("{}. {}", n, s),
            PurityBlock::Conclusion(s) => println!("End: {}", s),
        }
    }

    Ok(())
}

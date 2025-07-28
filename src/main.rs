use std::fs::File;
use std::io::Bytes;
#[warn(missing_docs)]
use std::io::prelude::*;

/// A purity file is a sequence of blocks surrounded by parens, brackets, curly brackets, or greater/less-than symbols
enum PurityBlock {
    PlainText(String),
    SubjectHeader(String, usize),
    Question(String, usize),
    Conclusion(String),
}

struct PurityParser<R: Read> {
    subject_number: usize,
    question_number: usize,
    source: Bytes<R>,
}

impl<R: Read> PurityParser<R> {
    pub fn new(input: R) -> Self {
        PurityParser {
            subject_number: 0,
            question_number: 0,
            source: input.bytes(),
        }
    }
}

#[derive(Copy, Clone)]
enum Character {
    Escaped(char),
    Unescaped(char),
}

fn read_char<R: Read>(stream: &mut std::io::Bytes<R>) -> Option<Character> {
    let byte = stream.next()?.ok()?;
    if byte as char == '\\' {
        Some(Character::Escaped(stream.next()?.ok()? as char))
    } else {
        Some(Character::Unescaped(byte as char))
    }
}

fn is_char_important(c: Character) -> (bool, char) {
    match c {
        Character::Escaped(c) => (false, c),
        Character::Unescaped(c) => match c {
            '[' | ']' | '(' | ')' | '{' | '}' | '<' | '>' => (true, c),
            other => (false, other),
        },
    }
}

fn push_until<R: Read>(stream: &mut std::io::Bytes<R>, c: char, s: &mut String) {
    // DO consume the ending char, since we know what it is and we don't want it
    while let x = is_char_important(read_char(stream).unwrap()) {
        if x == (true, c) {
            break;
        }
        s.push(x.1);
    }
}
impl<R: Read> Iterator for PurityParser<R> {
    type Item = PurityBlock;

    fn next(&mut self) -> Option<Self::Item> {
        // All out-of-block characters are treated as comments
        // I'm assuming ASCII since these test files are from 1989, TODO
        let mut char_stream = &mut self.source;

        // Discard extra comment/formatting bytes before blocks
        let mut char_of_importance: (bool, char);
        loop {
            char_of_importance = is_char_important(read_char(char_stream)?);
            if char_of_importance.0 {
                break;
            }
        }

        // Read the text inside the block
        let mut text: String = String::new();
        let close = match char_of_importance.1 {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => return None,
        };
        push_until(&mut char_stream, close, &mut text);

        /// Give back the block type
        match close {
            ')' => {
                self.question_number += 1;
                Some(PurityBlock::Question(text, self.question_number))
            }
            ']' => {
                self.subject_number += 1;
                Some(PurityBlock::SubjectHeader(text, self.subject_number))
            }
            '}' => Some(PurityBlock::PlainText(text)),
            '>' => Some(PurityBlock::Conclusion(text)),
            _ => None,
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = File::open("tests/hacker")?;
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

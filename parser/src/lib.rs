use std::io::prelude::*;

/// A purity file is a sequence of blocks surrounded by parens, brackets, curly brackets, or greater/less-than symbols
pub enum PurityBlock {
    PlainText(String),
    SubjectHeader(String, usize),
    Question(String, usize),
    Conclusion(String),
}

pub struct PurityParser<R: Read> {
    subject_number: usize,
    question_number: usize,
    source: CharacterSream<R>,
}

impl<R: Read> PurityParser<R> {
    pub fn new(input: R) -> Self {
        PurityParser {
            subject_number: 0,
            question_number: 0,
            source: CharacterSream::new(input.bytes()),
        }
    }
}

#[derive(Copy, Clone)]
struct Character {
    is_escaped: bool,
    char: char,
}
/// Abstract away the UTF-8 parsing errors until a good solution comes up
struct CharacterSream<R: Read> {
    stream: std::io::Bytes<R>,
}
impl<R: Read> CharacterSream<R> {
    fn new(stream: std::io::Bytes<R>) -> Self {
        CharacterSream { stream }
    }
}
impl<R: Read> Iterator for CharacterSream<R> {
    type Item = Character;
    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.stream.next()?.ok()?;
        if byte as char == '\\' {
            Some(Character {
                is_escaped: true,
                char: self.stream.next()?.ok()? as char,
            })
        } else {
            Some(Character {
                is_escaped: false,
                char: byte as char,
            })
        }
    }
}

fn is_char_important(c: Character) -> bool {
    matches!(
        (c.is_escaped, c.char),
        (false, '[' | ']' | '(' | ')' | '{' | '}' | '<' | '>')
    )
}

fn push_until<R: Read>(stream: &mut CharacterSream<R>, c: char, s: &mut String) {
    // DO consume the ending char, since we know what it is and we don't want it
    while let Some(uninc) = stream.next()
        && (uninc.is_escaped || uninc.char != c)
    {
        s.push(uninc.char);
    }
}

impl<R: Read> Iterator for PurityParser<R> {
    type Item = PurityBlock;

    fn next(&mut self) -> Option<Self::Item> {
        // All out-of-block characters are treated as comments
        // I'm assuming ASCII since these test files are from 1989, TODO
        let char_stream = &mut self.source;

        // Discard extra comment/formatting bytes before blocks
        let char_of_importance = char_stream.find(|c| is_char_important(*c))?.char;
        // Read the text inside the block
        let mut text: String = String::new();
        let close = match char_of_importance {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => return None,
        };
        push_until(char_stream, close, &mut text);

        // Give back the block type
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

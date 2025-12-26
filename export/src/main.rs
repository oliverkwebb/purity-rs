use purity_parser::*;
use std::fs::File;
use std::io::{BufReader, Read};
// *Gasp* DEPENDENCIES! (in the background: "Oh the humanity!!")

fn output_html<R: Read>(parser: PurityParser<R>) {
    println!("{}", include_str!("html-templates/header.txt"));

    for block in parser {
        match block {
            PurityBlock::PlainText(header) => {
                println!("<pre>{}</pre>", html_escape::encode_text(&header))
            }
            PurityBlock::SubjectHeader(s, n) => println!(
                "<h2>Section {}: {}</h2><hr>",
                n,
                html_escape::encode_text(&s)
            ),
            PurityBlock::Question(s, _) => println!(
                "<li><input type=\"checkbox\">{}</li>",
                html_escape::encode_text(&s)
            ),
            PurityBlock::Conclusion(s) => println!("<pre>{}</pre>", html_escape::encode_text(&s)),
        }
    }

    println!("{}", include_str!("html-templates/footer.txt"));
}

fn main() {
    for arg in std::env::args().skip(1) {
        let input = BufReader::new(File::open(arg).unwrap());
        output_html(PurityParser::new(input));
    }
}

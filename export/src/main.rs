use purity_parser::*;
use std::fs::File;
use std::io::Read;

fn output_html<R: Read>(parser: PurityParser<R>) {
    println!(
        r#"
	<!DOCTYPE html>
	<html>
		<head>
			<style>
				body {{
					font-family: sans-serif;
					max-width: 1000px;
					margin: auto;
				}}

				h2 {{
					text-align: center;
				}}
			</style>
		</head>
		<body>
		<ol>
		"#
    );

    for block in parser {
        match block {
            PurityBlock::PlainText(header) => println!("<pre>{}</pre>", header),
            PurityBlock::SubjectHeader(s, n) => println!("<h2>Section {}: {}</h2><hr>", n, s),
            PurityBlock::Question(s, n) => println!("<li><input type=\"checkbox\">{}</li>", s),
            PurityBlock::Conclusion(s) => println!("<pre>{}</pre>", s),
        }
    }

    println!(
        r#"
		</ol>
		</body>
	</html>
	"#
    );
}

fn main() {
    let mut argv = std::env::args();
    argv.next(); // Program Name
    let format = argv.next().unwrap();
    let input = File::open(argv.next().unwrap()).unwrap();
    output_html(PurityParser::new(input));
}

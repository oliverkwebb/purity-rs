use purity_parser::*;
use std::fs::File;
use std::io::Read;
// *Gasp* DEPENDENCIES! (in the background: "Oh the humanity!!")
use html_escape;

fn output_html<R: Read>(parser: PurityParser<R>) {
    println!(
        r#"
	<!DOCTYPE html>
	<html>
		<head>
			<style>
				body {{
					font-family: sans-serif;
					max-width: 800px;
					margin: auto;
					padding-bottom: 100px;
				}}

				h2 {{
					text-align: center;
				}}

				button {{
					width: 100%;
					font-size: 1.5rem;
					height: 40px;
				}}

				#res {{
					text-align: center;
					font-size: 1.3rem;
				}}
			</style>
		</head>
		<body>
		<ol>
		"#
    );

    for block in parser {
        match block {
            PurityBlock::PlainText(header) => println!("<pre>{}</pre>", html_escape::encode_text(&header)),
            PurityBlock::SubjectHeader(s, n) => println!("<h2>Section {}: {}</h2><hr>", n, html_escape::encode_text(&s)),
            PurityBlock::Question(s, _) => println!("<li><input type=\"checkbox\">{}</li>", html_escape::encode_text(&s)),
            PurityBlock::Conclusion(s) => println!("<pre>{}</pre>", html_escape::encode_text(&s)),
        }
    }

    println!(
        r#"
		</ol>
		<script>
			function showres() {{
				let stats = (x => [x.length, x.reduce((a, b) => a + b, 0)])([...document.querySelectorAll("input")].map(x => x.checked));
				document.getElementById("res").innerHTML = `you answered ${{stats[1]}} yes out of ${{stats[0]}} questions. Which makes your
				purity score ${{((stats[0]-stats[1]/stats[0])/100).toFixed(2)}}%`;
			}}
		</script>
		<button type="button" onclick="showres()">See Purity Score</button>
		<p id="res"></p>
		<hr>
		</body>
	</html>
	"#
    );
}

fn main() {
    let mut argv = std::env::args();
    argv.next(); // Program Name
    let input = File::open(argv.next().unwrap()).unwrap();
    output_html(PurityParser::new(input));
}

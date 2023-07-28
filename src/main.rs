
pub mod dom;
pub mod html_parser;

fn main() {
    let html = r#"
    <html>
        <head>
            <title>HTML Parser</title>
        </head>
        <body>
            <h1>HTML Parser</h1>
            <div id="main" class="test">
                <p>Hello <em>world</em>!</p>
                <p>Rust is <em>fun</em>!</p>
            </div>
        </body>
    </html>
    "#;

    let mut parser = html_parser::Parser {
        pos: 0,
        input: html.to_string(),
    };

    for i in 0..10 {
        println!("idx:{}, char: {}, pos:{}", i, parser.consume_char(), parser.pos);
    }
}

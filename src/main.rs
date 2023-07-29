
pub mod dom;
pub mod html_parser;
pub mod css_parser;

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

    let parser = html_parser::parse(html.to_string());
    println!("{:#?}", parser);
}

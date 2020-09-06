use comrak::{markdown_to_html, ComrakOptions};
use clap::{Arg, App, SubCommand};

fn main() {
    parse_args();
}

enum Options {
    OutFile(String),
    StyleFile(String)
}

fn parse_args() -> Result<(String, String, String), std::io::Error> {

    let args = std::env::args();

    if args.len() < 2 {
        usage();
    }

    let markdown = std::fs::read_to_string(infile).expect(format!("Could not read {}", infile));
    let styles: String;
    if let Ok(res) = std::fs::read_to_string(styles) {
        styles = res;
    }
    else {
        styles = std::fs::read_to_string("styles.css").expect("Could not find styles.css");
    };

    Ok((markdown, styles, outfile))
}

fn convert(markdown: String, styles: String, outfile: String) -> Result<(), std::io::Error> {

    let filename = "readme.md";
    let header = format!("
    <html>\n
    <head>\n
    <title>{}</title>\n
    <meta name='viewport' content='width=device-width, initial-scale=1'>\n
    </head>\n
    <body>\n
    <div id='content'>", filename);

    let footer = format!("
    </div>\n
    <style type='text/css'>{}</style>
    </body>
    </html>", styles);

    let mut options = ComrakOptions::default();
    options.render.github_pre_lang = true;
    options.render.unsafe_ = true;
    let content = markdown_to_html(&markdown, &options);
    let html = header + &content + &footer;

    std::fs::write(outfile, &html).expect(format!("Failed to write to {}", outfile));
    Ok(())
}

fn usage() {
    let options = "
    -o | --outfile  : The output file to save the generated html to (default is README.html)\n
    -s | --styles   : The styles file to use (default is styles.css)
    ";
    let program_header = "RUSTY-GFM-HTML: The Rusty Github Flavored Markdown HTML Generator!";
    let copyright = "Copyright (c) 2020 Nathaniel Graham";
    let instructions = "Usage: rusty-gfm-html FILE [OPTIONS]";

    println!("{}\n{}\n\n{}", program_header, instructions, options);
}

use comrak::{markdown_to_html, ComrakOptions};
use clap::{Arg, App};

fn main() {
    match parse_args() {    
        Ok((markdown, styles, outfile)) => { 
            match convert(markdown, styles, outfile) {
                Ok(_) => {},
                Err(message) => println!("{}", message)
            }
        },
        Err(message) => println!("{}", message)
    }
}

fn parse_args() -> Result<(String, String, String), std::io::Error> {

    let matches = App::new("RUSTY-GFM-HTML")
        .version("1.0")
        .about("The Rusty Github Flavored Markdown HTML Generator!")
        .author("Nathaniel Graham <nathaniel.graham@protonmail.com>")
        .arg(Arg::with_name("infile")
            .value_name("INFILE")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("outfile")
            .short("o")
            .long("outfile")
            .value_name("OUTFILE")
            .help("output file to save the generated html to")
            .default_value("README.html")
            .takes_value(true))
        .arg(Arg::with_name("styles")
            .short("s")
            .long("styles")
            .value_name("STYLES")
            .help("The styles file to use")
            .default_value("styles.css")
            .takes_value(true))
        .get_matches();
    
    let mdfile = matches.value_of("infile").unwrap();
    let markdown = std::fs::read_to_string(&mdfile)?;
    let stfile = matches.value_of("styles").unwrap();
    let styles = std::fs::read_to_string(stfile)?;
    let outfile = String::from(matches.value_of("outfile").unwrap());
    
    // TODO fix error handeling
    // match (markdown, styles, outfile) {
    //     (Ok(md), Ok(st), ot) => Ok((md, st, ot)),
    //     (Err(md), _, _) => Err(),
    //     (_, Err(st), _) => Err(st)
    // }

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

    std::fs::write(&outfile, &html).expect(&format!("Failed to write to {}", outfile));
    Ok(())
}
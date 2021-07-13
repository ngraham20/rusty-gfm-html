use clap::{Arg, App};

/// Parse the input arguments
pub fn parse_args() -> Result<(String, String, String, bool, bool, bool), std::io::Error> {

    // get the directory of the executable
    let mut expath = std::env::current_exe()?;
    expath.pop();
    let stylepath = &format!("{}/styles.css", expath.to_str().unwrap());

    // set up the application
    let matches = App::new("RUSTY-GFM-HTML")
        .version("0.1.0")
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
            .default_value("out.html")
            .takes_value(true))
        .arg(Arg::with_name("styles")
            .short("s")
            .long("styles")
            .value_name("STYLES")
            .help("The styles file to use")
            .default_value(stylepath)
            .takes_value(true))
        .arg(Arg::with_name("highlight_syntax")
            .short("c")
            .long("highlight_syntax")
            .value_name("HIGHTLIGHT_SYNTAX")
            .help("Highlight the syntax within code blocks")
            .takes_value(false))
        .arg(Arg::with_name("embed_images")
            .short("e")
            .long("embed-images")
            .value_name("EMBED_IMAGES")
            .help("Embed images directly into the output html with base64 encoding. This drastically increases the size of the document, but removes the need to distribute image assets along with it.")
            .takes_value(false))
        .arg(Arg::with_name("smart_punctuation")
            .short("p")
            .long("smart_punctuation")
            .value_name("SMART_PUNCTUATION")
            .help("Enable smart punctuation. Warning: This will use Unicode characters, which may not render properly on all devices.")
            .takes_value(false))
        .get_matches();
    
    // process arguments
    let mdfile = matches.value_of("infile").unwrap();
    let markdown = std::fs::read_to_string(&mdfile)?;
    let stfile = matches.value_of("styles").unwrap();
    let styles = std::fs::read_to_string(stfile)?;
    let outfile = String::from(matches.value_of("outfile").unwrap());
    let embed_images = matches.is_present("embed_images");
    let highlight_syntax = matches.is_present("highlight_syntax");
    let smart_punctuation = matches.is_present("smart_punctuation");
    
    // TODO fix error handeling
    // match (markdown, styles, outfile) {
    //     (Ok(md), Ok(st), ot) => Ok((md, st, ot)),
    //     (Err(md), _, _) => Err(),
    //     (_, Err(st), _) => Err(st)
    // }

    Ok((markdown, styles, outfile, embed_images, highlight_syntax, smart_punctuation))
}
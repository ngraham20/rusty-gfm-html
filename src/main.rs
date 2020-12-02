use regex::Captures;
use std::io::Read;
use comrak::{markdown_to_html, ComrakOptions};
use clap::{Arg, App};
use syntect::{html, parsing, highlighting};
use regex::Regex;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    match parse_args() {    
        Ok((markdown, styles, outfile, embed_images, highlight_syntax, interactive, smart_punctuation)) => { 
            match convert(markdown, styles, outfile, embed_images, highlight_syntax, interactive, smart_punctuation) {
                Ok(_) => {},
                Err(message) => println!("{}", message)
            }
        },
        Err(message) => println!("{}", message)
    }
}

fn parse_args() -> Result<(String, String, String, bool, bool, bool, bool), std::io::Error> {

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
        .arg(Arg::with_name("interactive")
            .short("i")
            .long("interactive")
            .value_name("INTERACTIVE")
            .help("Allow modifications to output interactively")
            .takes_value(false))
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
    let interactive = matches.is_present("interactive");
    let smart_punctuation = matches.is_present("smart_punctuation");
    
    // TODO fix error handeling
    // match (markdown, styles, outfile) {
    //     (Ok(md), Ok(st), ot) => Ok((md, st, ot)),
    //     (Err(md), _, _) => Err(),
    //     (_, Err(st), _) => Err(st)
    // }

    Ok((markdown, styles, outfile, embed_images, highlight_syntax, interactive, smart_punctuation))
}

fn embed_html(html: &String) -> Result<String, std::io::Error> {
    let re = Regex::new(r#"(?P<before><img src=")(?P<filepath>.*?)(?P<after>".*>)"#).unwrap();
    let result = re.replace_all(&html, |caps: &Captures| {
        println!("Captured image: {:?}", &caps["filepath"]);
        let img = caps["filepath"].to_owned();
        let filetype = Path::new(&img)
        .extension()
        .and_then(OsStr::to_str).unwrap();
        println!("{}", img);
        let mut f = std::fs::File::open(&img).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        format!("{}data:image/{};base64,{}{}",&caps["before"], filetype, base64::encode(buffer), &caps["after"])
    });
    Ok(String::from(result.as_ref()))
}

fn highlight_codeblock_syntax(html: &String, interactive: bool) -> Result<String, std::io::Error> {
    // Replace the code block with syntax highlighted code
    let re = Regex::new(r#"(?ms:<pre lang="(?P<language>\w+)"><code>(?P<code>.*?)</code></pre>)"#).unwrap();
    let result = re.replace_all(&html, |caps: &Captures| {
        let ps = parsing::SyntaxSet::load_defaults_newlines();
        let ts = highlighting::ThemeSet::load_defaults();
        let lang = &caps["language"];
        let syntax;
        if lang != "" && &lang.to_ascii_lowercase() != "html" {
            syntax = match ps.find_syntax_by_name(titlecase::titlecase(&lang).as_ref()) {
                Some(syntax) => {
                    println!("Highlighting for code block in language: {}", &lang);
                    syntax
                },
                _ => if interactive {
                    let mut filetype = String::new();
                    println!("Failed to get syntax for code block with language: {}. What is the file extension associated with this language?", &lang);
                    std::io::stdin().read_line(&mut filetype).expect("Failed to read user input.");
                    filetype = String::from(filetype.trim_end());
                    if let Some(syn) = ps.find_syntax_by_extension(&filetype) {
                        println!("Highlighting for code block in language: {}", &filetype);
                        syn
                    } else { 
                        ps.find_syntax_plain_text() }
                } else {
                    println!("Failed to find syntax for: {}. Skipping highlight step for this code block.", &lang);
                    println!("Use interactive mode ( -i ) to specify codeblock languages interactively.");
                    ps.find_syntax_plain_text() }
            };
        }
        else {
            println!("HTML doesn't highlight properly due to escaped characters. Skipping highlight step for this code block.");
            syntax = ps.find_syntax_plain_text();
        }
        // let syntax = ps.find_syntax_by_name("Bash").unwrap();
        // let syntax = ps.find_syntax_by_extension("bash").unwrap();
        let theme = &ts.themes["base16-ocean.light"];
        html::highlighted_html_for_string(&caps["code"], &ps, &syntax, &theme)
    });
        // the syntect version of a syntax highlighted code block
    Ok(String::from(result.as_ref()))
}

fn cleanup_codeblocks(html: &String) -> Result<String, std::io::Error> {
    let re = Regex::new(r#"&amp;"#).unwrap();
    let result = re.replace_all(&html, "&");

    Ok(String::from(result))
}

fn convert(markdown: String, styles: String, outfile: String, embed_images: bool, highlight_syntax: bool, interactive: bool, smart_punctuation: bool) -> Result<(), std::io::Error> {

    let header = format!("
    <html>\n
    <head>\n
    <title>{}</title>\n
    <meta name='viewport' content='width=device-width, initial-scale=1'>\n
    </head>\n
    <body>\n
    <div id='content'>", outfile);

    let footer = format!("
    </div>\n
    <style type='text/css'>{}</style>
    </body>
    </html>", styles);

    let mut options = ComrakOptions::default();

    if smart_punctuation {
        options.parse.smart = true;
    }
    options.render.github_pre_lang = true;
    options.render.unsafe_ = true;
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    
    let content = markdown_to_html(&markdown, &options);
    let mut html = header + &content + &footer;

    if highlight_syntax {
        html = highlight_codeblock_syntax(&mut html, interactive)?;
        html = cleanup_codeblocks(&mut html)?;
    }

    if embed_images {
        html = embed_html(&mut html)?;
    }

    std::fs::write(&outfile, &html)?;
    Ok(())
}
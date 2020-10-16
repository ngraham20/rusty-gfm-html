use regex::Captures;
use std::io::Read;
use comrak::{markdown_to_html, ComrakOptions};
use clap::{Arg, App};
use regex::Regex;
use std::path::Path;
use std::ffi::OsStr;

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
        .get_matches();
    
    // process arguments
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

fn parse_html(html: &String) -> Result<(), std::io::Error> {
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
    std::fs::write("test.html", &result.as_ref())?;
    Ok(())
}

fn convert(markdown: String, styles: String, outfile: String) -> Result<(), std::io::Error> {

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
    options.render.github_pre_lang = true;
    options.render.unsafe_ = true;
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    
    let content = markdown_to_html(&markdown, &options);
    let html = header + &content + &footer;

    parse_html(&html);

    std::fs::write(&outfile, &html)?;
    Ok(())
}
use std::io::Read;
use std::path::Path;
use std::ffi::OsStr;
use regex::{Captures, Regex};
use comrak::{markdown_to_html, ComrakOptions};
use syntect::{html, parsing, highlighting};

/// Embeds image sources as base64 strings. This is useful for single-file documentation
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

fn highlight_codeblock_syntax(html: &String, theme_path: &String) -> Result<String, std::io::Error> {
    // Replace the code block with syntax highlighted code
    let re = Regex::new(r#"(?ms:<pre lang="(?P<language>\w+)"><code>(?P<code>.*?)</code></pre>)"#).unwrap();
    let result = re.replace_all(&html, |caps: &Captures| {
        let ps = parsing::SyntaxSet::load_defaults_newlines();
        // let ts = highlighting::ThemeSet::load_defaults();
        let lang = &caps["language"];
        let syntax;
        if lang != "" {
            if let Some(title) = ps.find_syntax_by_name(&lang) { syntax = title; }
            else if let Some(title) = ps.find_syntax_by_name(titlecase::titlecase(&lang).as_ref()) { syntax = title; }
            else if let Some(ext) = ps.find_syntax_by_extension(&lang) { syntax = ext; }
            else {
                println!("Failed to find syntax for: {}. Skipping highlight step for this code block.", &lang);
                syntax = ps.find_syntax_plain_text() }
        }
        else {
            syntax = ps.find_syntax_plain_text();
        }
        let theme = match highlighting::ThemeSet::get_theme(&theme_path) {
            Ok(tm) => tm,
            _ => panic!("Theme file not found")
        };
        // let theme = &ts.themes["base16-ocean.light"];
        html::highlighted_html_for_string(&caps["code"], &ps, &syntax, &theme)
    });
        // the syntect version of a syntax highlighted code block
    Ok(String::from(result.as_ref()))
}

/// Cleans up HTML escape codes that were added in code blocks
fn cleanup_codeblocks(html: &String) -> Result<String, std::io::Error> {
    let result = html.replace(r"&amp;", "&").replace(r"&lt;", "<").replace(r"&gt;", ">").replace(r"&quot;", "\"");
    Ok(result)
}

pub fn convert(markdown: String, styles: String, theme: String, outfile: String, embed_images: bool, highlight_syntax: bool, smart_punctuation: bool) -> Result<(), std::io::Error> {

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
    
    let mut content = markdown_to_html(&markdown, &options);
    if highlight_syntax {
        content = cleanup_codeblocks(&content)?;
        content = highlight_codeblock_syntax(&mut content, &theme)?;
    }
    let mut html = header + &content + &footer;

    if embed_images {
        html = embed_html(&mut html)?;
    }

    std::fs::write(&outfile, &html)?;
    Ok(())
}
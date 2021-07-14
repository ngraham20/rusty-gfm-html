mod args;
mod convert;
use args::*;
use convert::*;

fn main() {
    match parse_args() {    
        Ok((markdown, styles, theme, outfile, embed_images, highlight_syntax, smart_punctuation)) => { 
            match convert(markdown, styles, theme, outfile, embed_images, highlight_syntax, smart_punctuation) {
                Ok(_) => {},
                Err(message) => println!("{}", message)
            }
        },
        Err(message) => println!("{}", message)
    }
}
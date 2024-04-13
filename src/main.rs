use std::fs::File;
use std::io::Write;
use std::{env, process};
use std::collections::HashMap;
use lazy_static::lazy_static;

mod lexer;
use lexer::{Lexer, Token};

mod parser;
use parser::Parser;

lazy_static! {
    static ref SUPPORTED_LANGS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("js", "Javascript");
        map.insert("ts", "Typescript");
        map
    };
}

fn main() {
    const USAGE: &str = "Usage: glowup-transpiler <path> [--out=<lang>] [--reverse]";

    let args: Vec<String> = env::args().collect();

    println!("glowup-transpiler v{}", env!("CARGO_PKG_VERSION"));

    if args.len() < 2 {
        eprintln!("{}", USAGE);
        process::exit(1);
    }
    let string_path: &str = &args[1];
    let mut lang: &str = "js";
    let mut reverse = false;
    for arg in args.iter().skip(2) {
        if arg.starts_with("--") {
            let matcher = arg.trim_start_matches("--").split("=").nth(0).unwrap();
            match matcher {
                "lang" => {
                    lang = arg.split('=').nth(1).unwrap_or("");
                    if !SUPPORTED_LANGS.contains_key(lang) && lang != "gu" {
                        eprintln!("Unsupported language: {}", lang);
                        process::exit(1);
                    }
                }

                "reverse" => {
                    reverse = true;
                }

                "help" => {
                    println!("{}", USAGE);
                    process::exit(0);
                }

                _ => {
                    eprintln!("Unknown argument: {}", arg);
                    process::exit(1);
                }
            }
        } else {
            eprintln!("Unknown argument: {}", arg);
            process::exit(1);
        }
    }

    // figure out if its a directory or a file
    let path = std::path::Path::new(string_path);
    if (!reverse && path.extension().unwrap() != "gu") || (reverse && path.extension().unwrap() != lang) {
        eprintln!("File extension does not match the language");
        process::exit(1);
    }
    if path.is_dir() {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                transpile_file(&path, lang, reverse);
            }
        }
    } else {
        let out = transpile_file(path, lang, reverse);
        let mut out_path = path.to_path_buf();
        if reverse {
            out_path.set_extension("gu");
        } else {
            out_path.set_extension(lang);
        }
        if let Ok(mut file) = File::create(out_path.clone()) {
            if let Err(err) = file.write_all(out.as_bytes()) {
                eprintln!("Error writing to file: {}", err);
            } else {
                println!("Output written to {}", out_path.display());
            }
        } else {
            eprintln!("Error creating file");
        }
    }
}

fn transpile_file (path: &std::path::Path, lang: &str, reverse: bool) -> String {
    if reverse {
        println!("Transpiling {} to {}", path.display(), "Glowup");
    } else {
        println!("Transpiling {} to {}", path.display(), SUPPORTED_LANGS.get(lang).unwrap());
    }
    let out = match lang {
        "js" => {
            transpile_js_ts(path, reverse)
        }
        "ts" => {
            transpile_js_ts(path, reverse)
        }
        _ => {
            panic!("Unsupported language: {}", lang)
        }
    };
    out
}

fn transpile_js_ts (path: &std::path::Path, reverse: bool) -> String {
    let file = std::fs::read_to_string(path).unwrap();
    let mut lexer = Lexer::new(&file);
    //array to store the output
    let mut output: Vec<Token> = vec![];

    loop {
        if let Some(token) = lexer.next_token() {
            output.push(token);
        } else {
            break;
        }
    }
    let mut parser = Parser::new(output, "js", reverse);
    parser.parse()
}
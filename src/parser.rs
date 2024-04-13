use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::lexer::Token;

lazy_static! {
    static ref LANG_MAP: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        let mut map = HashMap::new();

        let mut js_keywords = HashMap::new();
        js_keywords.insert("noCap", "true");
        js_keywords.insert("cap", "false");
        js_keywords.insert("lowkey", "console");
        js_keywords.insert("stan", "log");
        js_keywords.insert("sus", "warn");
        js_keywords.insert("crowed", "debug");
        js_keywords.insert("cringe", "error");
        js_keywords.insert("tea", "info");
        js_keywords.insert("yeet", "throw");
        js_keywords.insert("L", "reject");
        js_keywords.insert("W", "resolve");
        js_keywords.insert("fuckup", "Error");
        js_keywords.insert("ghosted", "return null");
        js_keywords.insert("itsGiving", "return");
        js_keywords.insert("skrt", "break");
        js_keywords.insert("holdUp", "async");
        js_keywords.insert("letItCook", "await");
        js_keywords.insert("grab", "require");
        js_keywords.insert("finesse", "import");
        js_keywords.insert("ships", "exports");
        js_keywords.insert("ship", "export");
        js_keywords.insert("fr", "assert");
        js_keywords.insert("outOfPocket", "Infinity");
        js_keywords.insert("dis", "this");
        js_keywords.insert("clapback", "yield");
        js_keywords.insert("finna", "confirm");
        js_keywords.insert("vibeOnEvent", "addEventListener");
        js_keywords.insert("highkey", "alert");
        js_keywords.insert("Bet", "Promise");
        js_keywords.insert("chill", "setTimeout");
        js_keywords.insert("sussy", "?");
        js_keywords.insert("sussier", "??");
        js_keywords.insert("fuckAround", "try");
        js_keywords.insert("findOut", "catch");
        js_keywords.insert("lit", "let");
        js_keywords.insert("be", "=");
        js_keywords.insert("vibeCheck", "==");
        js_keywords.insert("vibeCheckFr", "===");
        js_keywords.insert("ate", "=>");
        map.insert("js", js_keywords);
        map
    };
}

pub struct Parser<'a> {
    tokens: Vec<Token>,
    lang: &'a str,
    reverse: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, lang: &'a str, reverse: bool) -> Self {
        Parser { tokens, lang, reverse }
    }

    fn invert_map(map: Option<&'static HashMap<&str, &str>>) -> Option<HashMap<&'a str, &'a str>> {
        map.map(|map| {
            let mut inverted = HashMap::new();
            for (key, value) in map {
                inverted.insert(*value, *key);
            }
            inverted
        })
    }

    pub fn parse(&mut self) -> String {
        let mut output = String::new();
        for token in &self.tokens {
            match token {
                Token::Identifier(keyword) => {
                    let map;
                    if self.reverse {
                        map = Self::invert_map(LANG_MAP.get(self.lang));
                    } else {
                        map = LANG_MAP.get(self.lang).cloned();
                    }
                    if let Some(language_keywords) = map {
                        if let Some(js_keyword) = language_keywords.get(keyword.as_str()) {
                            output.push_str(js_keyword);
                        } else {
                            output.push_str(keyword);
                        }
                    } else {
                        output.push_str(keyword);
                    }
                }
                _ => output.push_str(&token.to_string()),
            }
        }
        output
    }
}
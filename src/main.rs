use std::env;
use reqwest;
use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Response {
    meta: Meta,
    data: Vec<Data>
}

#[derive(Deserialize, Debug)]
struct Meta {
    status: i32
}

#[derive(Deserialize, Debug)]
struct Data {
    slug: String,
    is_common: Option<bool>,
    tags: Vec<String>,
    jlpt: Vec<String>,
    japanese: Vec<Word>,
    senses: Vec<Sense>,
    attribution: Attribution
}

#[derive(Deserialize, Debug)]
struct Word {
    word: Option<String>,
    reading: Option<String>
}

#[derive(Clone, Deserialize, Debug)]
struct Sense {
    english_definitions: Vec<String>,
    parts_of_speech: Vec<String>,
    links: Vec<Link>,
    tags: Vec<String>,
    restrictions: Vec<String>,
    see_also: Vec<String>,
    antonyms: Vec<String>,
    source: Vec<Source>,
    info: Vec<String>,
}

#[derive(Clone, Deserialize, Debug)]
struct Link {
    text: String,
    url: String
}

#[derive(Deserialize, Debug)]
struct Attribution {
    jmdict: bool,
    jmnedict: bool,
    dbpedia: DBPedia
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum DBPedia {
    Bool(bool),
    String(String)
}

#[derive(Clone, Deserialize, Debug)]
#[serde(untagged)]
enum Source {
    String(String),
    Map(HashMap<String, String>)
}

fn print_optional(value: Option<&String>) {
    match value {
        Some(val) => print!("{}", val),
        None => print!("--")
    }
}

fn print_word_data(data: Data) {
    let first_japanese_word = data.japanese.get(0);
    first_japanese_word.map(|j| {
        print_optional(j.word.as_ref());
        print!(" | ");
        print_optional(j.reading.as_ref());
    });

    print!(" |");

    let senses = data.senses.iter().take(5).collect::<Vec<_>>();
    for sense in senses {
        let select_definitions = sense.english_definitions.iter().take(5).collect::<Vec<_>>();
        for definition in select_definitions {
            print!("| {} ", definition);
        }
        print!("|");
    }
    println!()
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: rusty_jisho <search term>");
        std::process::exit(1);
    }

    let term = &args[1];

    println!("Searching '{}'", term);

    let query_url = &format!("https://jisho.org/api/v1/search/words?keyword={}", term);
    let result = reqwest::blocking::get(query_url)?;

    let response: Response = result.json()?;
    let response_data = response.data;

    if response_data.len() > 0 {
        for data in response_data {
            print_word_data(data);
        }
    } else {
        println!("Nothing found.")
    }

    Ok(())
}

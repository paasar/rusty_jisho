use std::env;
use reqwest;
use reqwest::Error;
use serde::Deserialize;

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
    is_common: bool,
    tags: Vec<String>,
    jlpt: Vec<String>,
    japanese: Vec<Word>,
    senses: Vec<Sense>,
    attribution: Attribution
}

#[derive(Deserialize, Debug)]
struct Word {
    word: Option<String>,
    reading: String
}

#[derive(Deserialize, Debug)]
struct Sense {
    english_definitions: Vec<String>,
    parts_of_speech: Vec<String>,
    links: Vec<Link>,
    tags: Vec<String>,
    restrictions: Vec<String>,
    see_also: Vec<String>,
    antonyms: Vec<String>,
    source: Vec<String>,
    info: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Link {
    text: String,
    url: String
}

#[derive(Deserialize, Debug)]
struct Attribution {
    jmdict: bool,
    jmnedict: bool,
//    dbpedia: DBPedia
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum DBPedia {
    Bool,
    String
}

/*
fn print_response(result: &mut reqwest::Response) -> Result<(), std::io::Error> {
    println!("RESULT!");
    std::io::copy(result, &mut std::io::stdout())?;
    println!("END RESULT!");
    Ok(())
}
*/

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: rusty_jisho <search term>");
        std::process::exit(1);
    }

    let term = &args[1];

    println!("Searching '{}'", term);

    let query_url = &format!("http://beta.jisho.org/api/v1/search/words?keyword={}", term);
    let mut result = reqwest::get(query_url)?;

//    print_response(&mut result);

    let response: Response = result.json()?;
    let response_data = response.data;

    if response_data.len() > 0 {
        for data in response_data {
            println!("Slug {}", data.slug)
        }
    } else {
        println!("Nothing found.")
    }


    Ok(())
}

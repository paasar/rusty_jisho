#[macro_use]
extern crate serde_derive;
extern crate reqwest;
use reqwest::Error;


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
//    dbpedia: bool //TODO can be bool or String
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
    let query_url = "http://beta.jisho.org/api/v1/search/words?keyword=dog";
    let mut result = reqwest::get(query_url)?;

//    print_response(&mut result);

    let response: Response = result.json()?;
    let response_data = response.data;

    match response_data.get(0) {
        Some(d) => println!("Slug {:?}", d.slug),
        _ => println!("Gaa!")
    }

    println!("\n\nThat's all!");
    Ok(())
}

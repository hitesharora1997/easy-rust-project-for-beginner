use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragragh: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("How to write to json in Rust"),
        author: String::from("Hitesh"),
        paragragh: vec![
            Paragraph {
                name: String::from("first sentence"),
            },
            Paragraph {
                name: String::from("body of the paragragh"),
            },
            Paragraph {
                name: String::from("last paragraph"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("the json is {:?}", json);
}

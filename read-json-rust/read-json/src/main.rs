use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
    {
        "article": "The Hitchhiker's Guide to the Galaxy",
        "author": "Hitesh",
        "paragraph": [
            {
                "name": "starting sentence"
            },
            {
                "name": "body of the paragraph"
            },
            {
                "name":"ending of the paragraph"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);
    println!(
        "\n\n The name of the first paragarph is: {} \n Article name is : {}",
        parsed.paragraph[2].name, parsed.author
    );
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}

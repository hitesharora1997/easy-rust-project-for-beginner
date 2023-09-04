use serde::{Seriliize, Deserializer}

#[derive(Seriliize,Descrialize)]
struct  Paragraph{
    name: String
}

#[derive(Seriliize,Descrialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}


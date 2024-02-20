use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    title: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}

fn main() {
    let mut paragraphs: Vec<Paragraph> = Vec::new();
    paragraphs.push(Paragraph {
        name: "First paragraph".to_string(),
    });
    paragraphs.push(Paragraph {
        name: String::from("Second paragraph"),
    });
    paragraphs.push(Paragraph {
        name: "Third paragraph".to_string(),
    });
    paragraphs.push(Paragraph {
        name: String::from("Fourth paragraph"),
    });

    let article_1 = Article {
        author: "John Doe".to_string(),
        title: "My first article".to_string(),
        paragraphs,
    };

    // println!("{}", serde_json::to_string(&article_1).unwrap());
    let json: String = serde_json::to_string(&article_1).unwrap();
    println!("{}", json);
}

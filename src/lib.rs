use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Direct(Content),
    Global(Content),
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    text: String,
    file: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Login {
    name: String,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Cyan,
    Cerise,
    Purple,
}

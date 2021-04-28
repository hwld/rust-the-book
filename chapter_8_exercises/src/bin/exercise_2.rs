use std::io;

fn main() {
    println!("ピッグラテンに変換したい文字列を入力してください");

    let text = read_line();
    let result = pig_latin_text(&text);

    println!("\n変換後:{}", result);
}

fn read_line() -> String {
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("入力の読み込みに失敗しました");

    String::from(text.trim())
}

fn pig_latin_text(text: &str) -> String {
    let mut result = String::new();
    let mut last = 0;
    for (index, separator) in
        text.match_indices(|c: char| c == ' ' || c == '.' || c == '?' || c == '!' || c == ',')
    {
        if last != index {
            result.push_str(&pig_latin_word(&text[last..index]));
        }
        result.push_str(separator);
        last = index + separator.len();
    }
    if last < text.len() {
        result.push_str(&pig_latin_word(&text[last..]));
    }

    result
}

fn pig_latin_word(word: &str) -> String {
    let vowels = "aiueoAIUEO";

    if !word.is_ascii() {
        panic!("ASCII文字以外をサポートしていません");
    }

    let first = word.chars().nth(0).unwrap().to_lowercase().to_string();

    if vowels.contains(&first) {
        format!("{}{}", word, "hay")
    } else {
        format!("{}{}{}", &word[1..], first, "ay")
    }
}

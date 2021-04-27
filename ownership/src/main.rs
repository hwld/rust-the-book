fn first_word1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn return_reference(_: &String) -> &i32 {
    &0
}

#[allow(dead_code)]
fn code1() {
    let mut s = String::from("hello");
    let word = first_word2(&s);
    // 以下はエラー
    // s.clear();
    println!("{}", word);
}

#[allow(dead_code)]
fn code2() {
    let mut s = String::from("hello");
    let word = first_word1(&s);

    // 可変参照と不変参照が存在するのにエラーにならない？
    // 参照は最後に使用されたところでスコープが終わる。 &sは引数として渡されただけでその後使われていないので、その時点でスコープから外れてる?
    // code1がエラーになって、code2がエラーにならないのは、参照を受け取る関数が参照を返したときに、それが引き継がれるから？

    // https://doc.rust-jp.rs/book-ja/ch10-03-lifetime-syntax.html#ライフタイム省略
    // より、ライフタイムを省略すると、一つの参照の引数と、戻り値の参照の引数のライフタイムが同じとみなされる。
    // なので、first_word2は戻り値のスコープが引数のスコープと同じになり、s.clear()の時点で不変な参照が存在することになる。
    // 一方first_word1では、参照を返さないので引数は渡された後にはスコープを抜けることになり、s.clear()の時点で不変な参照は存在しない。
    s.clear();
    println!("{}", word);
}

#[allow(dead_code)]
fn code3() {
    let mut s = String::from("hello");
    // 適当な参照を返してみる
    let some = return_reference(&s);
    // 以下はエラーになる
    // s.clear();
    //　その参照を使用する
    println!("{}", some);
}

fn main() {
    code1();
    code2();
    code3();
}

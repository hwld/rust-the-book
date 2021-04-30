fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // String
    {
        let string1 = String::from("long string is long");
        let result = "a";
        {
            let string2 = String::from("xyz");
            // エラーになる。
            //result = longest(string1.as_str(), string2.as_str());
        }
        println!("The longest string is {}", result);
    }

    // 文字列リテラル
    {
        let string1 = "long string is long";
        let result;
        {
            let string2 = "xyz";
            // エラーにならない。
            // これは、文字列リテラルのライフタイムは'staticであり、これはプログラム全体の期間を表すため。
            result = longest(string1, string2);
        }
        println!("The longest string is {}", result);
    }
}

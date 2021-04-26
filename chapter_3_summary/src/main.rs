use std::io;

fn to_fahrenheit_from(celsius: f64) -> f64 {
    celsius / 5.0 * 9.0 + 32.0
}

fn to_celsius_from(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

#[allow(dead_code)]
fn temperature_main() {
    let celsius = 12.0;
    let fahrenheit = 40.0;
    println!("{}°C = {:.3}°F", celsius, to_fahrenheit_from(celsius));
    println!("{}°F = {:.3}°C", fahrenheit, to_celsius_from(fahrenheit));
}

fn fibo(index: i32) -> i32 {
    if index == 0 || index == 1 {
        return index;
    }
    return index - 1 + fibo(index - 2);
}

#[allow(dead_code)]
fn fibo_main() {
    println!("Enter the order of the fibonacci:");
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("input error");
    let n: i32 = n.trim().parse().expect("Please type a number");

    println!("{}", fibo(n));
}

#[allow(dead_code)]
fn sing_the_twelve_days_of_christmas() {
    let daily_gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a laying",
        "seven swans a swimming",
        "eight maids a milking",
        "nine ladies dancing",
        "ten lords a leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for day_index in 0..daily_gifts.len() {
        let day = match day_index + 1 {
            1 => "first",
            2 => "second",
            3 => "third",
            4 => "fourth",
            5 => "fifth",
            6 => "sixth",
            7 => "seventh",
            8 => "eighth",
            9 => "ninth",
            10 => "tenth",
            11 => "eleventh",
            12 => "twelfth",
            _ => panic!(""),
        };

        println!("On the {} of Christmas my true love gave to me", day);
        for gift in daily_gifts[0..=day_index].iter().rev() {
            println!("{}", gift);
        }
        println!("")
    }
}

fn main() {
    temperature_main();
    fibo_main();
    sing_the_twelve_days_of_christmas()
}

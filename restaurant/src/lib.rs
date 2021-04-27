mod front_of_house;

pub use front_of_house::hosting;
pub use front_of_house::serving::back_of_house::{Appetizer, Breakfast};

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // エラー
    // meal.seasonal_fruit = String::from("blueberries");

    let order = Appetizer::Soup;
    println!("order {:?}", order);
}

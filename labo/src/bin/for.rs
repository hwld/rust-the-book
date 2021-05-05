fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    for item in vec.iter() {
        println!("{}", item)
    }

    // forはinto_iterを呼ぶので、所有権を奪う
    for item in vec {
        println!("{}", item);
    }
}

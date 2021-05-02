use std::rc::Rc;

fn main() {
    let mut r1 = Rc::new(String::from("Share"));
    let r2 = Rc::clone(&r1);
    let r3 = Rc::clone(&r1);

    // 読み取り専用 
    // r1.clear();

    // Share と出力されることから、型Tを持つスマートポインタをTのように扱える？
    println!("r1: {:?}", r1);
    // cloneで新しいStringを作成しない。
    // 同一Stringに複数の所有者が存在する
    println!("{:?}", r1.as_ptr());
    println!("{:?}", r2.as_ptr());
    println!("{:?}\n", r2.as_ptr());

    let b1 = Box::new(String::from("Hello"));
    let b2 = b1.clone();

    // Hello と出力されることから、型Tを持つスマートポインタをTのように扱える？
    println!("b1: {:?}", b1);
    // cloneで新しいStringを生成する
    println!("{:?}", b1.as_ptr());
    println!("{:?}", b2.as_ptr());
}

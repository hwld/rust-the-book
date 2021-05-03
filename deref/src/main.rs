use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
} 

impl<T> Deref for MyBox<T> {
    type Target = T;

    // MyBox<T> と T　を同じように扱える
    // (*(MyBox<T>::deref())) = T

    // &MyBox<T> と　&T　を同じように扱える
    // &(*(MyBox<T>::deref())) = &T
    // &Tと&MyBox<T>を変換したいときに、コンパイラはこれを自動で行ってくれる。これを参照外し型強制という。
    fn deref(&self) -> &T {
        &self.0
    }
}


fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn re() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    let boxing = MyBox::new(MyBox::new(MyBox::new(String::from("Rust"))));
    // ネストしても実行できる。
    hello(&boxing);

    let b = MyBox::new(String::from("Hello"));
    // (*(b.deref)).clear() が実行されている。
    // clearは &mut selfを受け取るが、MyBoxのDerefMutを実装していないのでエラーになる
    //b.clear();
}

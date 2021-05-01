use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
} 

impl<T> Deref for MyBox<T> {
    type Target = T;

    // &MyBox<T> と　&T　を同じように扱える
    // &(*(MyBox<T>::deref())) = &T
    // 仮引数に&T、実引数に&MyBox<T>のようなときに、コンパイラはこれを自動で行ってくれる。これを参照外し型矯正という。
    fn deref(&self) -> &T {
        &self.0
    }
}


fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let boxing = MyBox::new(MyBox::new(MyBox::new(String::from("Rust"))));
    // ネストしても実行できる。
    hello(&boxing);
}

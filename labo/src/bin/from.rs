struct A {}
impl From<B> for A {
    fn from(_: B) -> Self {
        A {}
    }
}

struct B {}

trait Trait {
    fn foo(&self);
}

impl Trait for A {
    fn foo(&self) {
        println!("Hello A");
    }
}

impl Trait for B {
    fn foo(&self) {
        println!("Hello B");
    }
}

trait Trait2 {
    fn bar();
}

impl Trait2 for A {
    fn bar() {
        println!("trait2 A")
    }
}

impl Trait2 for B {
    fn bar() {
        println!("trait2 B")
    }
}

struct C {}

fn main() {
    // コンパイラは、プログラム中の情報からどこに実装されているfromメソッドを呼び出せばよいか推論する。
    // この場合、引数がBで、左辺がAなので、 from(B) -> A になる。
    // FromトレイトはFrom<T> for U { from(t: T) -> Self }; のようになっているので、
    // T = B, U = Self = A, と推論して、 From<B> for A, Aに実装されているFrom<B>のfromを呼び出す.
    let a :A = From::from(B{});

    // 引数がCで、左辺がAなので、 from(C) -> A になり、 Aに実装されているFrom<C>のfromを呼び出すが、存在しないのでエラーになる。
    // let a :A = From::from(C{});


    let b  = B{};
    let _c = C{};

    // 引数が&Aと&Bなので、それぞれA,Bに実装されているTraitのfooを呼び出す
    // これは、Trait:fooが&selfに対して呼び出されるからで、&selfが&Aであるなら、AにTraitが実装されていると推論する
    Trait::foo(&a);
    Trait::foo(&b);
    // CにTraitは実装されていない
    // Trait::foo(&_c);

    // Trait2::barのシグネチャの中に型を推論できる情報が存在しないので、明示的にどこに実装しているかを指定しなければ呼び出せない。
    // Trait2::bar();
    <A as Trait2>::bar();
    <B as Trait2>::bar();
}

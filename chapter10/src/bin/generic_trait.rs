fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Tがヒープデータの場合にcloneに時間がかかる可能性がある
#[allow(dead_code)]
fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    // Tの参照からTに解決するために、cloneを使用して、新しいデータを作成する
    let mut largest = list[0].clone();

    for item in list.clone() {
        // &[T]をcloneしても、それぞれの要素はT型ではなく&T型になる。
        // これは、それぞれの要素の参照を持つリストがcloneされているということなので、さらにcloneしてTを作る
        let item = item.clone();
        if item > largest {
            largest = item;
        }
    }

    largest
}

// target関数は、リストの要素を返すので、リストの要素の参照を返すようにできる。
#[allow(dead_code)]
fn target_reference<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("{}", result);
}

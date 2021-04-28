use std::{collections::HashMap, io};

fn find_mean(nums: &Vec<i32>) -> f64 {
    let sum: i32 = nums.iter().sum();
    sum as f64 / nums.len() as f64
}

fn find_median(nums: &Vec<i32>) -> i32 {
    let mut nums = nums.clone();
    nums.sort();

    let mid = nums.len() / 2;
    if nums.len() % 2 == 0 {
        (nums[mid - 1] + nums[mid]) / 2
    } else {
        nums[mid]
    }
}

fn find_mode(nums: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for num in nums {
        let count = counts.entry(*num).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut max_count = 0;
    for (num, count) in &counts {
        if *count > max_count {
            mode = *num;
            max_count = *count;
        }
    }

    mode
}

// https://codereview.stackexchange.com/questions/173338/calculate-mean-median-and-mode-in-rust を参考に
#[allow(dead_code)]
fn average(numbers: &Vec<i32>) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

#[allow(dead_code)]
fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) / 2
    } else {
        numbers[mid]
    }
}

#[allow(dead_code)]
fn mode(numbers: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}
//

fn main() {
    let mut nums: Vec<i32> = Vec::new();

    loop {
        println!("格納する数を入力してください (q終了)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力の読み込みに失敗しました");
        let input = input.trim();

        if input == "q" {
            println!("入力を終了します");
            break;
        }

        let num: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数字かqを入力してください");
                continue;
            }
        };

        nums.push(num);
    }

    println!("mean is {}", find_mean(&nums));
    println!("median is {}", find_median(&nums));
    println!("mode is {}", find_mode(&nums));
}

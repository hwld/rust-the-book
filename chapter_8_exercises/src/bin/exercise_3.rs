use std::{fmt, io};

use company::{Department, DepartmentMap, Departments, Member};

enum Action {
    Quit,
    AddMember(String, String),
    PrintMembers(Department),
    PrintAllMembers,
    None,
}

fn main() {
    let mut departments = Departments::new();

    loop {
        match read_action() {
            Action::AddMember(department, name) => {
                departments.add_member(&department, &name);
                println!("部署 {} に メンバー {} を追加しました", department, name);
            }
            Action::PrintMembers(department) => {
                if let Some(members) = departments.get_members(&department) {
                    print_member(&department, &members);
                } else {
                    println!("部署にメンバーが存在しませんでした");
                }
            }
            Action::PrintAllMembers => {
                print_all_member(departments.get_map());
            }
            Action::Quit => {
                println!("終了します");
                break;
            }
            Action::None => {
                println!("不正なコマンドです");
                print_help();
                continue;
            }
        }
    }
}

fn print_help() {
    println!("----------------------------------------------");
    println!("使用可能なコマンド");
    println!("Add <department-name> to <member-name>");
    println!("Print <department-name>");
    println!("Print all-member");
    println!("Quit");
    println!("----------------------------------------------");
}

fn print_all_member(map: &DepartmentMap) {
    let mut departments: Vec<(&Department, &Vec<Member>)> = map.iter().collect();
    departments.sort_by_key(|&(department, _)| department);

    for (department, members) in departments {
        print_member(department, members);
    }
}

fn print_member(department: &str, members: &Vec<Member>) {
    println!("----------------------------------");
    println!("部署: {}", department);
    println!("メンバー:");
    for mem in members {
        println!("    {}", mem);
    }
    println!("----------------------------------");
}

fn read_action() -> Action {
    println!("");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("入力を読み取ることができませんでした");
    let input = input.trim();

    let word: Vec<&str> = input.split_whitespace().collect();

    if word.len() == 4 && word[0] == "Add" && word[2] == "to" {
        return Action::AddMember(String::from(word[1]), String::from(word[3]));
    } else if word.len() == 2 && word[0] == "Print" {
        if word[1] == "all-member" {
            return Action::PrintAllMembers;
        }
        return Action::PrintMembers(String::from(word[1]));
    } else if word.len() == 1 && word[0] == "Quit" {
        return Action::Quit;
    }

    Action::None
}

mod company {
    use std::collections::HashMap;

    pub type Department = String;
    pub type Member = String;
    pub type DepartmentMap = HashMap<Department, Vec<Member>>;
    pub struct Departments {
        map: DepartmentMap,
    }

    impl Departments {
        pub fn new() -> Departments {
            Departments {
                map: HashMap::new(),
            }
        }

        pub fn add_member(&mut self, department: &str, name: &str) {
            let names = self
                .map
                .entry(String::from(department))
                .or_insert(Vec::new());
            names.push(String::from(name));
        }

        pub fn get_members(&self, department: &str) -> Option<&Vec<Member>> {
            self.map.get(department)
        }

        pub fn get_map(&self) -> &DepartmentMap {
            &self.map
        }
    }
}

use serde::{Deserialize, Serialize};
use serde_json;
extern crate termion;

use std::fs::File;
use std::io::{stdin, Read};

#[derive(Debug, Serialize, Deserialize)]
struct Node {

}

#[derive(Debug, Serialize, Deserialize)]
struct NodeOption {
    
}

#[derive(Debug, Serialize, Deserialize)]
struct Character {
    name: String,
    race: String,
    sub_race: Option<String>,
    gender: String,
    sexuality: Option<String>,
    age: i32,
    traits: Vec<String>,
    inventory: Vec<Item>
}

impl Character {
    fn new(character_name: String) -> Self {
        let mut character = String::new();
        File::open(format!("src/assets/character/{}.json", character_name))
            .unwrap()
            .read_to_string(&mut character)
            .unwrap();
        serde_json::from_str(&character).unwrap()
    }

    fn add_trait(&mut self, new_trait: String) {
        self.traits.push(new_trait);
    }

    fn remove_trait(&mut self, old_trait: String) {
        let index = self.traits.iter().position(|r| -> bool {r == &old_trait}).unwrap_or_else(|| {
            panic!("Failed to remove trait, trait not found.");});
        self.traits.remove(index);
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Item {
    name: String,
    amount: i16
}

#[derive(Debug, Deserialize, Serialize)]
struct Opt {
    key: isize,
    value: String,
}

fn main() {
    printbar();
    let inp = getinput();
    println!("{}", inp);
    mainmenu();
    let mut rarity = Character::new("rarity".to_string());
    println!("{}", rarity.traits[0]);
    if rarity.traits.contains(&"fashionista".to_string()) {
        println!("{} is cute!", rarity.name);
    };
    rarity.add_trait("cute".to_string());
    println!("{:?}", rarity);
    rarity.remove_trait("cute".to_string());
    println!("{:?}", rarity);
    if rarity.inventory.contains(&Item { name: "hat".to_string(), amount: 10 }) {
        println!("test")
    }
}

fn printbar() {
    let (x, _) = termion::terminal_size().unwrap();
    println!("{:-<1$}", "", x as usize);
}

fn getinput() -> String {
    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line.");
    input_string
}

fn get_selected() -> i8 {
    let mut selected_option = String::new();

    std::io::stdin()
        .read_line(&mut selected_option)
        .expect("failed to read line");

    selected_option.trim().parse().unwrap_or_else(|_| {
        eprintln!("please enter a valid option");
        get_selected()
    })
}

fn mainmenu() {
    let res: Vec<Opt> = parse_json();
    for opt in res {
        println!("{}) {}", opt.key, opt.value);
    }
    let inp = get_selected();
    match inp {
        1 => println!("hi1"),
        2 => println!("hi2"),
        3 => println!("hi3"),
        4 => println!("hi4"),
        _ => {
            eprintln!("please enter a valid option");
            mainmenu();
        }
    }
}

fn parse_json() -> Vec<Opt> {
    let mut file = String::new();
    File::open("src/main_menu.json")
        .unwrap()
        .read_to_string(&mut file)
        .unwrap();
    let file: Vec<Opt> = serde_json::from_str(&file).unwrap();
    file
}

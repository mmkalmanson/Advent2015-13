use clap::Parser;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/// Advent 2015-10
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the part to solve
    #[clap(short, long)]
    part: u8,
}
#[derive(Debug)]
struct Setup {
    seating: [&'static str; 9],
    happiness: isize,
}

fn setup(instructions: HashMap<(&str, &str), isize>) -> HashMap<String, HashMap<String, isize>> {
    let mut people: HashMap<String, HashMap<String, isize>> = HashMap::new();

    for ((person, partner), change) in instructions {
        let change_vec = people.entry(person.to_string()).or_insert(HashMap::new());
        change_vec.insert(partner.to_string(), change);
    }

    let x = people
        .into_iter()
        .map(|(x, mut y)| {
            y.insert("Me".to_string(), 0);
            (x, y)
        })
        .collect();

    x
}

fn permutations() -> Vec<[&'static str; 9]> {
    let perm = [
        "Alice", "David", "Bob", "Carol", "George", "Frank", "Mallory", "Eric", "Me",
    ]
    .into_iter()
    .permutations(9);
    let i: Vec<[&'static str; 9]> = perm.map(|p| p.try_into().unwrap()).collect();
    i
}

fn table_setup_calculate(
    setup: [&'static str; 9],
    changes: &HashMap<String, HashMap<String, isize>>,
) -> Setup {
    let mut happiness = 0;
    for (i, person) in setup.iter().enumerate() {
        let mut person_change = 0;
        let x = changes.get(&person.to_string());
        if let Some(change) = x {
            if let Some(person_left) = setup.get(i - 1) {
                let y = change.get(&person_left.to_string()).unwrap();
                person_change += y;
            } else {
                let y = change.get(&setup.last().unwrap().to_string()).unwrap();
                person_change += y;
            }
            if let Some(person_right) = setup.get(i + 1) {
                let y = change.get(&person_right.to_string()).unwrap();
                person_change += y;
            } else {
                let y = change.get(&setup[0].to_string()).unwrap();
                person_change += y;
            }
        }
        happiness += person_change;
    }
    Setup {
        seating: setup,
        happiness,
    }
}
fn main() {
    let args = Args::parse();
    let input = include_str!("../input.txt");
    let distances: HashMap<(&str, &str), isize> = HashMap::from_iter(input.lines().map(|line| {
        let (person, distance_partner) = line.split_once(' ').expect("Error");
        let (distance, partner) = distance_partner.split_once(' ').expect("Num + Partner");
        let distance: isize = distance.parse().expect("every count is a number");
        ((person, partner), distance)
    }));

    let people = setup(distances);

    let mut list_of_happinesses: Vec<Setup> = Vec::new();
    for x in permutations() {
        list_of_happinesses.push(table_setup_calculate(x, &people));
    }

    match args.part {
        1 => {
            // list_of_happinesses.sort_by(|x, y| x.happiness.cmp(&y.happiness));
            // for x in list_of_happinesses
            let x = list_of_happinesses
                .iter()
                .reduce(|x, y| if x.happiness > y.happiness { x } else { y })
                .expect("Gotta be something");
            {
                println!("{:?}", x);
            }
        }
        2 => {}
        _ => eprintln!("Invalid part"),
    }
}

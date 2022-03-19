use std::env;

use rand::{Rng, prelude::ThreadRng};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 { // Not enough arguments given
        println!("Not enough arguments!");
        println!("Need two numbers, one for amount and one for max height");
        return;
    }

    // Store amount of buildings user entered
    let amount: u32 = args.get(1)
        .unwrap()
        .parse()
        .unwrap();

    // Max height of buildings, since it is random this height may not be reached
    let max_height: u32 = args.get(2)
        .unwrap()
        .parse()
        .unwrap();

    loop { // Ask to quit until user types 'q', otherwise print a new city each time
        println!("Quit? Type 'q'");
        let mut input = String::new();

        std::io::stdin()
                .read_line(&mut input)
                .expect("Unable to read input!");

        if input.eq("q\n") { // q and newline character
            break; // If they type q, exit loop now and therefore end program
        }

        gen_buildings(&amount, &max_height); // Print out new city
    }
}

fn gen_buildings(amount: &u32, max_height: &u32) {
    let mut heights = Vec::new(); // List of building heights
    let mut tallest_height = 0; // Tallest building, used to avoid printing redundant lines

    let mut r = rand::thread_rng();
    for _ in 0..*amount {
        let num = r.gen_range(0..*max_height);

        heights.push(num);
        if num > tallest_height {
            tallest_height = num;
        }
    }

    // Go down from *tallest_height* to 0 printing each building at or above this height
    while tallest_height > 0 {
        let i = tallest_height;

        for h in &mut *heights {
            print_building(&mut r, h, &i);
        }
        println!(); // Go to new line

        tallest_height -= 1;
    }
}

fn print_building(rng: &mut ThreadRng, bldg_height: &u32, current_height: &u32) {
    use std::cmp::Ordering;

    let rand_chance = rng.gen::<f32>();

    match bldg_height.cmp(current_height) {
        Ordering::Less => { // If building is too short then draw nothing (the sky)
            if rand_chance > 0.08 {
                print!("    ");
            } else { // 8% chance for there to be a star:
                print!(" *  ");
            }
        },
        Ordering::Equal => { // If building is at this height, draw the "tip"
            if rand_chance > 0.25 {
                print!(" ___");
            } else {
                print!(" .|."); // 25% chance for a pointy roof
            }
        },
        Ordering::Greater => { // If building is above, draw the walls
            if rand_chance > 0.25 {
                print!(" | |");
            } else {
                print!(" |#|"); // 25% chance for a window
            }
        },
    }
}

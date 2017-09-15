extern crate rand;

use std::io;
use rand::distributions::{Range, IndependentSample};
use rand::{thread_rng, Rng};


const NUM_DOORS: u8 = 3;
const SWITCH: usize = 0;
const NO_SWITCH: usize = 1;

fn generate_index() -> u8 {
    let range: Range<u8> = Range::new(0, NUM_DOORS);
    range.ind_sample(&mut rand::thread_rng())
}

fn generate_presenter_door(car_door: u8, chosen_door: u8) -> u8 {
    let mut possible_doors: Vec<u8> = Vec::new();
    for i in 0..NUM_DOORS {
        if i as u8 != car_door && i as u8 != chosen_door {
            possible_doors.push(i as u8);
        }
    }

    let v: &[u8] = possible_doors.as_slice();
    let mut rng = thread_rng();
    *rng.choose(v).unwrap()
}

fn play_game() -> (usize, bool) {
    let car_door = generate_index();
    let chosen_door = generate_index();
    let presenter_door = generate_presenter_door(car_door, chosen_door);

    let door_switch_decision: usize = Range::new(0, 2).ind_sample(&mut rand::thread_rng());

    let final_chosen_door = if door_switch_decision == SWITCH {
        !(presenter_door | chosen_door) & 0x3 //selects the missing door
    } else {
        chosen_door
    };

    (door_switch_decision as usize, final_chosen_door == car_door)
}

fn main() {
    println!("Number of runs: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let attempts: u64 = input.trim().parse::<u64>().unwrap();

    let mut results: [u64; 2] = [0; 2];
    let mut num_runs: [u64; 2] = [0; 2];
    for _ in 0..attempts {
        let (door_switch_decision, correct_door): (usize, bool) = play_game();
        results[door_switch_decision] = results[door_switch_decision] + correct_door as u64;
        num_runs[door_switch_decision] = num_runs[door_switch_decision] + 1;
    }

    println!("Average results:\nDid not switch doors: {}\nSwitched doors: {}",
             results[NO_SWITCH] as f64 / num_runs[NO_SWITCH] as f64,
             results[SWITCH] as f64 / num_runs[SWITCH] as f64);
}

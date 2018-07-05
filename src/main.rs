extern crate rand;
use rand::prelude::*;
use std::env;

const ERROR_MESSAGE: &str = "nat 1";

/*
    Generates random numbers from user input in the form "_d_".
    For example, roll 2d8 d6 rolls 2d8 and prints their values & sum, then rolls 1d6 and prints its value.

    James McClung, July 2018
*/
fn main() {
    let mut rng = thread_rng();

    // all the (number of rolls, die to roll) pairs
    // e.g. 2d8, d6 becomes [[2,8], [1,6]]
    let xdys: Vec<Vec<u32>> =
        env::args()
        .skip(1)
        .map(|arg| arg
            .split('d')
            .map(|arg| {
                if arg == "" {
                    1
                } else {
                   arg.parse().expect(ERROR_MESSAGE)
                }
            })
            .collect())
        .collect();

    // roll the dice and print the results
    for xdy in xdys {
        print!("Rolling {}d{}:", xdy[0], xdy[1]);
        if xdy[0] == 1 { // just print the value
            println!(" {}", roll(&mut rng, xdy[1]));
        } else { // print all the values and the sum
            let mut num_rolled;
            let mut sum = 0;
            for _ in 0..xdy[0] { // the values
                num_rolled = roll(&mut rng, xdy[1]);
                print!(" {}", num_rolled);
                sum = sum + num_rolled;
            }
            println!(" -> {}", sum);
        }
    }

}

/*
    Generates a random number using rng between 1 and die, inclusive.
*/
fn roll(rng: &mut impl Rng, die: u32) -> u32 {
    rng.gen_range(1, die+1)
}

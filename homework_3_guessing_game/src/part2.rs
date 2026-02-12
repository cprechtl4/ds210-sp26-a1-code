use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE.
        if player.ask_to_compare(min) == 0 {
            return min;
        }

        let mut min = min;
        let mut max = max;
        
        loop {
            let guess = min + ((max - min) / 2);
            let result = player.ask_to_compare(guess);

            if result == 0 {
                return guess;
            }else if result == 1 {
                min = guess + 1;
            }else if result == -1 {
                max = guess - 1;
            }
        } 
    }
}
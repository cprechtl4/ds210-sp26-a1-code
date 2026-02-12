use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE.
        let mut min = min;
        let mut max = max;

        loop {
            let i = (max - min) / 2;
            let result = player.ask_to_compare(i);

            if result == 0 {
                return i;
            }else if result == 1 {
                min = i + 1;
            }else if result == -1 {
                max = i - 1;
            }
        }
    }
}
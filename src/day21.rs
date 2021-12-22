use std::collections::HashMap;
use std::io;

use crate::common;

type State = (usize, usize, usize, usize, bool);

fn calculate_universes(universes: &mut HashMap<State, (usize, usize)>, state: &State) {
    const UNIVERSE_COUNTS: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];
    if !universes.contains_key(state) {
        let mut result = (0, 0);
        for throw in 3..10 {
            let next_state = if state.4 {
                (
                    state.0,
                    state.1 + (state.3 + throw) % 10 + 1,
                    state.2,
                    (state.3 + throw) % 10,
                    false,
                )
            } else {
                (
                    state.0 + (state.2 + throw) % 10 + 1,
                    state.1,
                    (state.2 + throw) % 10,
                    state.3,
                    true,
                )
            };

            if next_state.0 >= 21 {
                result.0 += UNIVERSE_COUNTS[throw - 3];
            } else if next_state.1 >= 21 {
                result.1 += UNIVERSE_COUNTS[throw - 3];
            } else {
                calculate_universes(universes, &next_state);
                let next_result = universes[&next_state];
                result.0 += next_result.0 * UNIVERSE_COUNTS[throw - 3];
                result.1 += next_result.1 * UNIVERSE_COUNTS[throw - 3];
            }
        }
        universes.insert(*state, result);
    }
}

pub fn main(do_b: bool) -> io::Result<usize> {
    let mut lines = common::read_lines("inputs/21.txt")?;

    let mut player_1_pos = lines.next().unwrap()?.trim()[28..]
        .parse::<usize>()
        .unwrap()
        - 1;
    let mut player_2_pos = lines.next().unwrap()?.trim()[28..]
        .parse::<usize>()
        .unwrap()
        - 1;

    if do_b {
        let state = (0, 0, player_1_pos, player_2_pos, false);

        let mut winners: HashMap<State, (usize, usize)> = HashMap::new();

        calculate_universes(&mut winners, &state);

        let (universes_player_1, universes_player_2) = winners[&state];
        Ok(universes_player_1.max(universes_player_2))
    } else {
        let mut roll_count = 0;
        let mut die_state = 0;

        let mut player_1_score = 0;
        let mut player_2_score = 0;

        macro_rules! roll {
            ($die_state:ident) => {{
                let roll = $die_state + 1;
                roll_count += 1;
                $die_state = ($die_state + 1) % 100;
                roll
            }};
        }

        loop {
            let roll_1 = roll!(die_state);
            let roll_2 = roll!(die_state);
            let roll_3 = roll!(die_state);
            let roll = roll_1 + roll_2 + roll_3;
            player_1_pos = (player_1_pos + roll) % 10;
            player_1_score += player_1_pos + 1;

            if player_1_score >= 1000 {
                break;
            }

            let roll_1 = roll!(die_state);
            let roll_2 = roll!(die_state);
            let roll_3 = roll!(die_state);
            let roll = roll_1 + roll_2 + roll_3;
            player_2_pos = (player_2_pos + roll) % 10;
            player_2_score += player_2_pos + 1;

            if player_2_score >= 1000 {
                break;
            }
        }

        Ok(player_1_score.min(player_2_score) * roll_count)
    }
}

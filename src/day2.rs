use std::iter::zip;

fn get_action_pairs(input: Vec<String>) -> Vec<(u8, u8)> {
    let action_pairs = input.iter()
        // Split by spaces to get a pair of letters    
        .map(|l| l.split_once(' ').unwrap())
        // Convert the first letter from a range A-C to 0-2, and
        // the second from X-Z. Start by converting to a byte.
        .map(|(left, right)| (left.as_bytes()[0], right.as_bytes()[0]) )
        // A = ASCII 0x41, X = ASCII 0x58
        .map(|(left, right)| (left - 0x41, right - 0x58))
        .collect::<Vec<(u8, u8)>>();
    
    action_pairs
}

pub fn part1(input: Vec<String>) -> String {
    let action_pairs = get_action_pairs(input);
    
    // In the pair list, rock is 0. The action scores need to be incremented by 1.
    let player_action_scores = action_pairs.iter().map(|(_, player_action)| player_action + 1);

    // Using some modular arithmetic, this will result in 0 for a draw, 1 for a loss, 2 for a win
    // The actions are unsigned, so add 3 to prevent overflow when subtracting.
    let outcome_scores = action_pairs.iter().map(|(opponent_action, player_action)| (opponent_action + 3 - player_action) % 3)
        // It would be nice to have loss = 0, draw = 1. The map x -> 2x + 1 in ℤ/3ℤ achieves this swap and leaves win as 2.
        .map(|outcome| ((outcome * 2) + 1) % 3)
        // And scale as needed
        .map(|outcome| 3 * outcome);
    
    let total_score : i32 = zip(player_action_scores, outcome_scores)
        // sneaking in a cheeky lil cast
        .map(|(action_score, outcome_score)| (action_score + outcome_score) as i32)
        .sum();
    
    total_score.to_string()
}

pub fn part2(input: Vec<String>) -> String {
    let action_pairs = get_action_pairs(input);

    // This time, the second number is the outcome, and it just needs to be scaled.
    let outcome_scores = action_pairs.iter().map(|(_, outcome)| outcome * 3);

    // By reversing the outcome calculation from Part 1, we can infer the player action
    // Since x -> 2x + 1 just swaps 0 and 1 in ℤ/3ℤ, it is its own inverse. Thanks, abstract algebra!
    let player_action_scores = action_pairs.iter()
        .map(|(opponent_action, outcome)| (opponent_action, ((2 * outcome) + 1) % 3))
        // Rearranging Part 1's `opponent_action - player_action = outcome`, we get `opponent_action - outcome = player_action`.
        // Again, taking care to avoid overflow.
        .map(|(opponent_action, outcome)| (opponent_action + 3 - outcome) % 3)
        // Finally, reïndex the player actions so that Rock is 1 and not 0
        .map(|player_action| player_action + 1);

    let total_score : i32 = zip(player_action_scores, outcome_scores)
        // sneaking in a cheeky lil cast
        .map(|(action_score, outcome_score)| (action_score + outcome_score) as i32)
        .sum();
    
    total_score.to_string()
}
use nom::{
    character::complete::newline,
    multi::{many_m_n, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::collections::{HashSet, VecDeque};
use std::fs;

type Cards = VecDeque<usize>;

pub fn run() {
    let input = fs::read_to_string("day22.txt").unwrap();
    println!("22:1 {}", run_1(&input));
    println!("22:2 {}", run_2(&input));
}

fn parse_player(i: &str) -> IResult<&str, Vec<usize>> {
    let re = regex::Regex::new(r#"(Player \d:)"#).unwrap();
    let (i, _) = nom_regex::str::re_capture(re)(i)?;
    let (i, _) = newline(i)?;
    separated_list1(newline, crate::helper::uval)(i)
}

fn parse(i: &str) -> IResult<&str, (Cards, Cards)> {
    let (i, (p1, p2)) = separated_pair(parse_player, many_m_n(2, 2, newline), parse_player)(i)?;
    Ok((i, (p1.into_iter().collect(), p2.into_iter().collect())))
}

fn run_1(input: &str) -> usize {
    let (_, (mut p1, mut p2)) = parse(input).unwrap();

    while !p1.is_empty() && !p2.is_empty() {
        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();
        if p1_card > p2_card {
            p1.push_back(p1_card);
            p1.push_back(p2_card);
        } else {
            p2.push_back(p2_card);
            p2.push_back(p1_card);
        }
    }

    let winner = if p1.is_empty() { p2 } else { p1 };

    winner.into_iter().rev().zip(1..).map(|(a, b)| a * b).sum()
}

fn play_game(game: &mut usize, mut p1_cards: Cards, mut p2_cards: Cards) -> (Cards, Cards) {
    println!("=== Game {} ===", game);
    let mut previous_hands = HashSet::new();
    let mut round = 1;
    let this_game = game.clone();
    *game += 1;
    while !p1_cards.is_empty() && !p2_cards.is_empty() {
        println!("\n-- Round {} (Game {}) --", round, this_game);
        println!("Player 1's deck: {:?}", p1_cards);
        println!("Player 2's deck: {:?}", p2_cards);
        if previous_hands.contains(&p1_cards) {
            println!("Player 1 wins round {} of game {}!", round, this_game);

            // Since p1 wins the GAME, return his cards and no cards for p2
            return (p1_cards, VecDeque::new());
        }

        previous_hands.insert(p1_cards.clone());

        let p1 = p1_cards.pop_front().unwrap();
        let p2 = p2_cards.pop_front().unwrap();
        println!("Player 1 plays: {}", p1);
        println!("Player 2 plays: {}", p2);
        if p1_cards.len() >= p1 && p2_cards.len() >= p2 {
            println!("Playing a sub-game to determine the winner...");
            let p1_subgame_cards = p1_cards.iter().take(p1).map(|c| *c).collect();
            let p2_subgame_cards = p2_cards.iter().take(p2).map(|c| *c).collect();
            let (_p1_subgame_cards, p2_subgame_cards) =
                play_game(game, p1_subgame_cards, p2_subgame_cards);
            if p2_subgame_cards.is_empty() {
                println!("Player 1 wins round {} of game {}!", round, this_game);
                p1_cards.push_back(p1);
                p1_cards.push_back(p2);
            } else {
                println!("Player 2 wins round {} of game {}!", round, this_game);
                p2_cards.push_back(p2);
                p2_cards.push_back(p1);
            }
        } else {
            if p1 > p2 {
                println!("Player 1 wins round {} of game {}!", round, this_game);
                p1_cards.push_back(p1);
                p1_cards.push_back(p2);
            } else {
                println!("Player 2 wins round {} of game {}!", round, this_game);
                p2_cards.push_back(p2);
                p2_cards.push_back(p1);
            }
        }
        round += 1;
    }
    (p1_cards, p2_cards)
}

fn run_2(input: &str) -> usize {
    let (_, (mut p1, mut p2)) = parse(input).unwrap();
    let mut game = 1;

    while !p1.is_empty() && !p2.is_empty() {
        let p = play_game(&mut game, p1, p2);
        p1 = p.0;
        p2 = p.1;
    }
    println!("== Post-game results ==");
    println!("Player 1 cards: {:?}", &p1);
    println!("Player 2 cards: {:?}", &p2);
    let winner = if p1.is_empty() { p2 } else { p1 };

    winner.into_iter().rev().zip(1..).map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn aoc22_run_1() {
        assert_eq!(super::run_1(INPUT_1), 306);
    }

    #[test]
    fn aoc22_run_2() {
        assert_eq!(super::run_2(INPUT_1), 291);
    }
}

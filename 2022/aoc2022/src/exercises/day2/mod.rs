use anyhow::{anyhow, Result};
use log::info;
use std::convert::TryFrom;

pub fn process_input(filedata: &String) -> Tournament {
    Tournament::from(filedata)
}

pub fn run_part1(filedata: &String) -> usize {
    let mut tournament = process_input(filedata);
    part1(&mut tournament);
    info!("Day 2 Answer: {:?}", tournament.player2);
    tournament.player2
}

pub fn part1(tournament: &mut Tournament) -> usize {
    tournament.play_tournament_p1();
    tournament.player2
}

pub fn run_part2(filedata: &String) -> usize {
    let mut tournament = process_input(filedata);
    part2(&mut tournament);
    info!("Day 2 Answer: {:?}", tournament.player2);
    tournament.player2
}

pub fn part2(tournament: &mut Tournament) -> usize {
    tournament.play_tournament_p2();
    tournament.player2
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day2::run_part1;
    use crate::exercises::day2::run_part2;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file("./src/exercises/day2/input_test.txt")),
            15
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day2/input.txt")),
            13009
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day2/input_test.txt")),
            12
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day2/input.txt")),
            10398
        )
    }
}

#[derive(Debug, Clone)]
enum Handshape {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Handshape {
    type Error = anyhow::Error;
    fn try_from(raw_shape: &str) -> Result<Self> {
        match raw_shape {
            "A" => Ok(Handshape::Rock),
            "B" => Ok(Handshape::Paper),
            "C" => Ok(Handshape::Scissors),
            "X" => Ok(Handshape::Rock),
            "Y" => Ok(Handshape::Paper),
            "Z" => Ok(Handshape::Scissors),
            _ => Err(anyhow!("Invalid handshape detected")),
        }
    }
}

#[derive(Debug, Clone)]
enum Strategy {
    Rock,
    Paper,
    Scissors,
    Lose,
    Draw,
    Win,
}

impl TryFrom<&str> for Strategy {
    type Error = anyhow::Error;
    fn try_from(raw_shape: &str) -> Result<Self> {
        match raw_shape {
            "A" => Ok(Strategy::Rock),
            "B" => Ok(Strategy::Paper),
            "C" => Ok(Strategy::Scissors),
            "X" => Ok(Strategy::Lose),
            "Y" => Ok(Strategy::Draw),
            "Z" => Ok(Strategy::Win),
            _ => Err(anyhow!("Invalid strategy detected")),
        }
    }
}

#[derive(Debug, Clone)]
struct Round {
    playersp1: Vec<Handshape>,
    playersp2: Vec<Strategy>,
}

impl From<&str> for Round {
    fn from(raw_round: &str) -> Self {
        Round {
            playersp1: raw_round
                .split(" ")
                .map(|raw_shape| Handshape::try_from(raw_shape).unwrap())
                .collect::<Vec<Handshape>>(),

            playersp2: raw_round
                .split(" ")
                .map(|raw_strategy| Strategy::try_from(raw_strategy).unwrap())
                .collect::<Vec<Strategy>>(),
        }
    }
}

impl Round {
    fn play_round_p1(&self, player1: &mut usize, player2: &mut usize) {
        let points_win = 6;
        let points_draw = 3;
        let points_lose = 0;
        let points_rock = 1;
        let points_paper = 2;
        let points_scissors = 3;

        match self.playersp1[0] {
            Handshape::Rock => match self.playersp1[1] {
                Handshape::Rock => {
                    *player1 += points_draw;
                    *player1 += points_rock;
                    *player2 += points_draw;
                    *player2 += points_rock;
                }
                Handshape::Paper => {
                    *player1 += points_lose;
                    *player1 += points_rock;
                    *player2 += points_win;
                    *player2 += points_paper;
                }
                Handshape::Scissors => {
                    *player1 += points_win;
                    *player1 += points_rock;
                    *player2 += points_lose;
                    *player2 += points_scissors;
                }
            },
            Handshape::Paper => match self.playersp1[1] {
                Handshape::Rock => {
                    *player1 += points_win;
                    *player1 += points_paper;
                    *player2 += points_lose;
                    *player2 += points_rock;
                }
                Handshape::Paper => {
                    *player1 += points_draw;
                    *player1 += points_paper;
                    *player2 += points_draw;
                    *player2 += points_paper;
                }
                Handshape::Scissors => {
                    *player1 += points_lose;
                    *player1 += points_paper;
                    *player2 += points_win;
                    *player2 += points_scissors;
                }
            },
            Handshape::Scissors => match self.playersp1[1] {
                Handshape::Rock => {
                    *player1 += points_lose;
                    *player1 += points_scissors;
                    *player2 += points_win;
                    *player2 += points_rock;
                }
                Handshape::Paper => {
                    *player1 += points_win;
                    *player1 += points_scissors;
                    *player2 += points_lose;
                    *player2 += points_paper;
                }
                Handshape::Scissors => {
                    *player1 += points_draw;
                    *player1 += points_scissors;
                    *player2 += points_draw;
                    *player2 += points_scissors;
                }
            },
        }
    }

    fn play_round_p2(&self, player1: &mut usize, player2: &mut usize) {
        let points_win = 6;
        let points_draw = 3;
        let points_lose = 0;
        let points_rock = 1;
        let points_paper = 2;
        let points_scissors = 3;

        match self.playersp2[0] {
            Strategy::Rock => match self.playersp2[1] {
                Strategy::Lose => {
                    *player1 += points_win;
                    *player1 += points_rock;
                    *player2 += points_lose;
                    *player2 += points_scissors;
                }
                Strategy::Draw => {
                    *player1 += points_draw;
                    *player1 += points_rock;
                    *player2 += points_draw;
                    *player2 += points_rock;
                }
                Strategy::Win => {
                    *player1 += points_lose;
                    *player1 += points_rock;
                    *player2 += points_win;
                    *player2 += points_paper;
                }
                _ => panic!("invalid strategy for player 2"),
            },
            Strategy::Paper => match self.playersp2[1] {
                Strategy::Lose => {
                    *player1 += points_win;
                    *player1 += points_paper;
                    *player2 += points_lose;
                    *player2 += points_rock;
                }
                Strategy::Draw => {
                    *player1 += points_draw;
                    *player1 += points_paper;
                    *player2 += points_draw;
                    *player2 += points_paper;
                }
                Strategy::Win => {
                    *player1 += points_lose;
                    *player1 += points_paper;
                    *player2 += points_win;
                    *player2 += points_scissors;
                }
                _ => panic!("invalid strategy for player 2"),
            },
            Strategy::Scissors => match self.playersp2[1] {
                Strategy::Lose => {
                    *player1 += points_win;
                    *player1 += points_scissors;
                    *player2 += points_lose;
                    *player2 += points_paper;
                }
                Strategy::Draw => {
                    *player1 += points_draw;
                    *player1 += points_scissors;
                    *player2 += points_draw;
                    *player2 += points_scissors;
                }
                Strategy::Win => {
                    *player1 += points_lose;
                    *player1 += points_scissors;
                    *player2 += points_win;
                    *player2 += points_rock;
                }
                _ => panic!("invalid strategy for player 2"),
            },
            _ => panic!("invalid strategy for player 1"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Tournament {
    player1: usize,
    player2: usize,
    games: Vec<Round>,
}

impl From<&String> for Tournament {
    fn from(encrypted_strategy_guide: &String) -> Self {
        Tournament {
            player1: 0,
            player2: 0,
            games: encrypted_strategy_guide
                .split("\n")
                .map(|raw_round| Round::from(raw_round))
                .collect::<Vec<Round>>(),
        }
    }
}

impl Tournament {
    fn play_tournament_p1(&mut self) {
        for game in &self.games {
            game.play_round_p1(&mut self.player1, &mut self.player2)
        }
    }

    fn play_tournament_p2(&mut self) {
        for game in &self.games {
            game.play_round_p2(&mut self.player1, &mut self.player2)
        }
    }
}

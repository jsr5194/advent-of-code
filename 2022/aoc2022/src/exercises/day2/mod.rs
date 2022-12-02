use log::info;
use std::fs;

fn get_input(filename: &str) -> Tournament {
    let contents_str = fs::read_to_string(&filename).expect("could not read file");
    Tournament::from(contents_str)
}

pub fn run_part1(filename: &str) -> usize {
    let mut tournament = get_input(&filename);
    tournament.play_tournament();
    info!("Day 1 Answer: {:?}", tournament.player2);
    tournament.player2
}

pub fn run_part2(filename: &str) {
    info!("running part 2");
}

#[cfg(test)]
mod tests {
    use crate::exercises::day2::run_part1;
    use crate::exercises::day2::run_part2;
    #[test]
    fn test() {
        assert_eq!(run_part1("./src/exercises/day2/input_test.txt"), 15);
        assert_eq!(run_part1("./src/exercises/day2/input.txt"), 13009)
    }
}

#[derive(Debug, Clone)]
enum Handshape {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Handshape {
    fn from(raw_shape: &str) -> Self {
        match raw_shape {
            "A" => Handshape::Rock,
            "B" => Handshape::Paper,
            "C" => Handshape::Scissors,
            "X" => Handshape::Rock,
            "Y" => Handshape::Paper,
            "Z" => Handshape::Scissors,
            _ => panic!("Invalid handshape detected"),
        }
    }
}

#[derive(Debug, Clone)]
struct Round {
    players: Vec<Handshape>,
}

impl From<&str> for Round {
    fn from(raw_round: &str) -> Self {
        Round {
            players: raw_round
                .split(" ")
                .map(|raw_shape| Handshape::from(raw_shape))
                .collect::<Vec<Handshape>>(),
        }
    }
}

impl Round {
    fn play_round(&self, player1: &mut usize, player2: &mut usize) {
        let points_win = 6;
        let points_draw = 3;
        let points_lose = 0;
        let points_rock = 1;
        let points_paper = 2;
        let points_scissors = 3;

        match self.players[0] {
            Handshape::Rock => match self.players[1] {
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
            Handshape::Paper => match self.players[1] {
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
            Handshape::Scissors => match self.players[1] {
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
}

#[derive(Debug, Default, Clone)]
struct Tournament {
    player1: usize,
    player2: usize,
    games: Vec<Round>,
}

impl From<String> for Tournament {
    fn from(encrypted_strategy_guide: String) -> Self {
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
    fn play_tournament(&mut self) {
        for game in &self.games {
            game.play_round(&mut self.player1, &mut self.player2)
        }
    }
}

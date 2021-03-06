extern crate rand;
extern crate strum;
extern crate serde;

use crate::events::Player;
use fltk::{button::Button, output::Output, prelude::*};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use serde::{Serialize, Deserialize};

type PlayCards = Vec<Card>;

const R_A: u8 = 1;
const R_J: u8 = 11;
const R_Q: u8 = 12;
const R_K: u8 = 13;

#[derive(Debug, EnumIter, Copy, Clone, Serialize, Deserialize)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(Copy, Clone, Debug)]
pub enum WinStatus {
    Pisti,
    Win,
    Pass,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> core::fmt::Result {
        let x = match self {
            Suit::Spade => "♠︎",
            Suit::Heart => "♡",
            Suit::Diamond => "♢",
            Suit::Club => "♣︎",
        };
        write!(f, "{}", x)
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Card {
    pub rank: u8,
    pub suit: Suit,
}

#[derive(Debug)]
pub struct Game {
    pub board: PlayCards,
    pub deck: PlayCards,
    pub player1_hand: PlayCards,
    pub player2_hand: PlayCards,
    pub player1_won_cards: PlayCards,
    pub player2_won_cards: PlayCards,
    pub player1_pisti_count: u8,
    pub player2_pisti_count: u8,
    pub first_player: Player,
    pub ai_player: Option<Player>, // if there are two human players set this None
    pub player1_point: usize,
    pub player2_point: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: vec![],
            deck: vec![],
            player1_hand: vec![],
            player2_hand: vec![],
            player1_won_cards: vec![],
            player2_won_cards: vec![],
            player1_pisti_count: 0,
            player2_pisti_count: 0,
            first_player: Player::Player1,
            ai_player: Some(Player::Player1),
            player1_point: 0,
            player2_point: 0,
        }
    }
    pub fn create_deck(&mut self) {
        let mut deck_vec: PlayCards = Vec::with_capacity(52);
        for a_suit in Suit::iter() {
            for a_rank in 1..14 {
                deck_vec.push(Card {
                    rank: a_rank,
                    suit: a_suit,
                })
            }
        }
        self.deck = deck_vec
    }

    pub fn shuffle_deck(&mut self) {
        if self.deck.len() > 0 {
            let mut rng = thread_rng();
            self.deck.shuffle(&mut rng);
        }
    }

    pub fn give_cards_to_players(&mut self) {
        if self.deck.len() > 7 {
            for _i in 0..4 {
                self.player1_hand.push(self.deck.pop().unwrap());
                self.player2_hand.push(self.deck.pop().unwrap());
            }
        }
    }

    pub fn put_cards_onto_board(&mut self) {
        if self.deck.len() > 3 {
            for _i in 0..4 {
                self.board.push(self.deck.pop().unwrap());
            }
        }
    }

    pub fn pick_card_for_ai(&mut self) -> usize {
        let ai_cards = self.get_ai_player_hand().unwrap();
        let mut tmp_i = get_random_index(&self.player2_hand);

        return if self.board.len() > 0 {
            let card_on_board = self.board.last().unwrap();
            for i in 0..ai_cards.len() {
                let pp_card = ai_cards.get(i).unwrap();
                if pp_card.rank == card_on_board.rank {
                    tmp_i = i;
                }
            }
            tmp_i
        } else {
            tmp_i
        };
    }

    pub fn play_card(&mut self, a_card: Card) -> WinStatus {
        self.board.push(a_card);
        let board_len = self.board.len();
        return if board_len > 1 {
            let last_card_1 = self.board.get(board_len - 1).unwrap();
            let last_card_2 = self.board.get(board_len - 2).unwrap();
            if last_card_1.rank == last_card_2.rank {
                if board_len == 2 {
                    WinStatus::Pisti
                } else {
                    WinStatus::Win
                }
            } else if last_card_1.rank == R_J {
                WinStatus::Win
            } else {
                WinStatus::Pass
            }
        } else {
            WinStatus::Pass
        };
    }

    pub fn create_pisti(&mut self, stat: WinStatus, player: Player) {
        match stat {
            WinStatus::Pisti => {
                println!("Pisti!!!");
                match player {
                    Player::Player1 => {
                        self.player1_pisti_count += 1;
                    }
                    Player::Player2 => {
                        self.player2_pisti_count += 1;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn move_cards_if_win(&mut self, stat: WinStatus, player: Player) {
        match stat {
            WinStatus::Pisti | WinStatus::Win => match player {
                Player::Player1 => {
                    self.player1_won_cards.append(&mut self.board);
                    self.create_pisti(stat, player);
                }
                Player::Player2 => {
                    self.player2_won_cards.append(&mut self.board);
                    self.create_pisti(stat, player);
                }
            },
            WinStatus::Pass => {}
        }
        self.calculate_points();
    }

    pub fn get_last_player(&self) -> Player {
        match self.first_player {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }

    pub fn is_reshuffle_required(&self) -> bool {
        match self.board.last() {
            None => false,
            Some(a_card) => a_card.rank == R_J,
        }
    }

    pub fn get_player_cards(&self, a_player: Player) -> &PlayCards {
        match a_player {
            Player::Player1 => &self.player1_hand,
            Player::Player2 => &self.player2_hand,
        }
    }

    pub fn get_ai_player_hand(&self) -> Option<&PlayCards> {
        match self.ai_player {
            None => None,
            Some(a_player) => Some(self.get_player_cards(a_player)),
        }
    }

    pub fn calculate_points(&mut self) {
        let player_1_card_count_score =
            if self.player1_won_cards.len() > self.player2_won_cards.len() {
                3
            } else {
                0
            };
        let player_2_card_count_score =
            if self.player2_won_cards.len() > self.player1_won_cards.len() {
                3
            } else {
                0
            };
        self.player1_point = (self.player1_pisti_count * 10) as usize + player_1_card_count_score;
        self.player2_point = (self.player2_pisti_count * 10) as usize + player_2_card_count_score;
        for p_card in &self.player1_won_cards {
            if p_card.rank == R_A || p_card.rank == R_J {
                self.player1_point += 1;
            } else if p_card.rank == 2
                && match p_card.suit {
                    Suit::Club => true,
                    _ => false,
                }
            {
                self.player1_point += 2;
            } else if p_card.rank == 10
                && match p_card.suit {
                    Suit::Diamond => true,
                    _ => false,
                }
            {
                self.player1_point += 3;
            }
        }
        for p_card in &self.player2_won_cards {
            if p_card.rank == R_A || p_card.rank == R_J {
                self.player2_point += 1;
            } else if p_card.rank == 2
                && match p_card.suit {
                    Suit::Club => true,
                    _ => false,
                }
            {
                self.player2_point += 2;
            } else if p_card.rank == 10
                && match p_card.suit {
                    Suit::Diamond => true,
                    _ => false,
                }
            {
                self.player2_point += 3;
            }
        }
    }
}

pub fn get_random_index(a_vec: &PlayCards) -> usize {
    (rand::random::<f32>() * a_vec.len() as f32).floor() as usize
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", rank_to_str(self.rank), self.suit)
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {}{} ", rank_to_str(self.rank), self.suit)
    }
}

pub fn rank_to_str(a_rank: u8) -> String {
    return match a_rank {
        R_A => String::from("A"),
        R_J => String::from("J"),
        R_Q => String::from("Q"),
        R_K => String::from("K"),
        _ => a_rank.to_string(),
    };
}

pub fn update_ui_on_button_press(
    ai_cards: &Vec<Button>,
    pl_cards: &Vec<Button>,
    board: &Button,
    the_game: &Game,
    out1: &Output,
    out2: &Output,
) {
    let cards_len = the_game.player2_hand.len();
    for i in 0..4 {
        let ai_but = ai_cards.get(i).unwrap();
        let pl_but = pl_cards.get(i).unwrap();
        if i < cards_len {
            let a_string = format!("{}", the_game.player2_hand.get(i).unwrap());
            ai_but.to_owned().set_label(&a_string);
            let b_string = format!("{}", the_game.player1_hand.get(i).unwrap());
            pl_but.to_owned().set_label(&b_string);
            pl_but.to_owned().activate();
            pl_but.to_owned().clear_visible_focus();
        } else {
            ai_but.to_owned().set_label("");
            ai_but.to_owned().deactivate();
            pl_but.to_owned().set_label("");
            pl_but.to_owned().deactivate();
            pl_but.to_owned().clear_visible_focus();
        }
    }
    if the_game.board.len() > 0 {
        let c_string = format!("{}", the_game.board.last().unwrap());
        board.to_owned().set_label(&c_string);
    } else {
        board.to_owned().set_label("");
    }
    let o_string1 = format!(
        "{} Pist({}) PT({})",
        the_game.player1_won_cards.len(),
        the_game.player1_pisti_count,
        the_game.player1_point,
    );
    let o_string2 = format!(
        "{} Pist({}) PT({})",
        the_game.player2_won_cards.len(),
        the_game.player2_pisti_count,
        the_game.player2_point,
    );
    out1.to_owned().set_value(&o_string1);
    out2.to_owned().set_value(&o_string2);
    println!("player1_won_cards: {:#?}", the_game.player1_won_cards);
    println!("player2_won_cards: {:#?}", the_game.player2_won_cards);
}

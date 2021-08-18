extern crate rand;
extern crate strum;

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::fmt;
use std::io;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type PlayCards = Vec<Card>;

const R_A: u8 = 1;
const R_J: u8 = 11;
const R_Q: u8 = 12;
const R_K: u8 = 13;

#[derive(Debug, EnumIter, Copy, Clone)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(Copy, Clone, Debug)]
enum WinStatus {
    Pisti,
    Win,
    Pass,
}

#[derive(Copy, Clone, Debug)]
enum Player {
    Player1,
    Player2,
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

struct Card {
    rank: u8,
    suit: Suit,
}

#[derive(Debug)]
struct Game {
    board: PlayCards,
    deck: PlayCards,
    player1_hand: PlayCards,
    player2_hand: PlayCards,
    player1_won_cards: PlayCards,
    player2_won_cards: PlayCards,
    player1_pisti_count: u8,
    player2_pisti_count: u8,
    first_player: Player,
    ai_player: Option<Player>, // if there are two human players set this None
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
        }
    }
    fn create_deck(&mut self) {
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

    fn shuffle_deck(&mut self) {
        if self.deck.len() > 0 {
            let mut rng = thread_rng();
            self.deck.shuffle(&mut rng);
        }
    }

    fn give_cards_to_players(&mut self) {
        if self.deck.len() > 7 {
            for _i in 0..4 {
                self.player1_hand.push(self.deck.pop().unwrap());
                self.player2_hand.push(self.deck.pop().unwrap());
            }
        }
    }

    fn put_cards_onto_board(&mut self) {
        if self.deck.len() > 3 {
            for _i in 0..4 {
                self.board.push(self.deck.pop().unwrap());
            }
        }
    }

    fn pick_card_for_ai(&mut self) -> usize {
        return if self.board.len() > 0 {
            get_random_index(&self.player2_hand)
        } else {
            get_random_index(&self.player2_hand)
        };
    }

    fn play_card(&mut self, a_card: Card) -> WinStatus {
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

    fn create_pisti(&mut self, stat: WinStatus, player: Player) {
        match stat {
            WinStatus::Pisti => {
                println!("Pisti!!!");
                match player {
                    Player::Player1 => {self.player1_pisti_count += 1;}
                    Player::Player2 => {self.player2_pisti_count += 1;}
                }
            }
            _ => {}
        }
    }

    fn move_cards_if_win(&mut self, stat: WinStatus, player: Player) {
        match stat {
            WinStatus::Pisti | WinStatus::Win => {
                match player {
                    Player::Player1 => {
                        self.player1_won_cards.append(&mut self.board);
                        self.create_pisti(stat, player);
                    }
                    Player::Player2 => {
                        self.player2_won_cards.append(&mut self.board);
                        self.create_pisti(stat, player);
                    }
                }
            }
            WinStatus::Pass => {}
        }
    }

    fn get_last_player(&self) -> Player {
        match self.first_player {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }

    fn is_reshuffle_required(&self) -> bool{
        match self.board.last() {
            None => {false}
            Some(a_card) => {
                a_card.rank == R_J
            }
        }
    }
}

fn get_random_index(a_vec: &PlayCards) -> usize {
    (rand::random::<f32>() * a_vec.len() as f32).floor() as usize
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {}{} ", rank_to_str(self.rank), self.suit)
    }
}

fn rank_to_str(a_rank: u8) -> String {
    return match a_rank {
        R_A => String::from("A"),
        R_J => String::from("J"),
        R_Q => String::from("Q"),
        R_K => String::from("K"),
        _ => a_rank.to_string(),
    };
}

fn main() {
    // use Suit::*;
    use Player::*;
    use WinStatus::*;
    let mut my_game = Game::new();
    my_game.create_deck();
    my_game.shuffle_deck();
    my_game.put_cards_onto_board();
    while my_game.is_reshuffle_required()  {
        println!("J is the top card on board. Reshuffling");
        my_game.create_deck();
        my_game.shuffle_deck();
        my_game.put_cards_onto_board();
    }
    my_game.give_cards_to_players();
    // println!("game state after cards handed: {:#?}", my_game);
    println!("board: {:#?}", my_game.board);
    loop {
        println!(
            "player cards: {}, ai card: {}",
            my_game.player1_hand.len(),
            my_game.player2_hand.len()
        );
        if my_game.player1_hand.len() < 1 && my_game.player2_hand.len() < 1 {
            if my_game.deck.len() > 7 {
                my_game.give_cards_to_players();
            } else {
                break;
            }
        }

        'input_loop: loop {
            println!("Please pick a card: {:#?}", my_game.player1_hand);

            let mut is_valid = false;
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_n) => {
                    match input.trim().parse::<usize>() {
                        Ok(number) => {
                            if number <= my_game.player1_hand.len() {
                                is_valid = true;
                                let a_card = my_game.player1_hand.remove(number - 1);
                                println!("you played: {}", a_card);
                                let stat = my_game.play_card(a_card);
                                my_game.move_cards_if_win(stat, Player1);
                            }
                        }
                        Err(ee) => eprintln!("{}", ee),
                    };
                }
                Err(error) => println!("error: {}", error),
            }
            if is_valid {
                break 'input_loop;
            }
        }

        let ai_card_index = my_game.pick_card_for_ai();
        let a_card = my_game.player2_hand.remove(ai_card_index);
        println!("ai played: {}", a_card);
        let stat = my_game.play_card(a_card);
        my_game.move_cards_if_win(stat, Player2);
        println!("board: {:#?}", my_game.board);
    }

    // last player get all remaining cards on board
    my_game.move_cards_if_win(Win, my_game.get_last_player());

    println!(
        "player won {} cards ({} pisti), ai won {} cards ({} pisti)",
        my_game.player1_won_cards.len(),
        my_game.player1_pisti_count,
        my_game.player2_won_cards.len(),
        my_game.player2_pisti_count
    )
}

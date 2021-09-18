mod events;
mod game;
mod network;

use std::{thread, time::Duration};

use events::*;

use fltk::{app, button::Button, output::Output, prelude::*, window::Window};

use game::*;

fn main() {
    use Player::*;
    use WinStatus::*;
    let mut my_game = Game::new();
    my_game.create_deck();
    my_game.shuffle_deck();
    my_game.put_cards_onto_board();
    while my_game.is_reshuffle_required() {
        println!("J is the top card on board. Reshuffling");
        my_game.create_deck();
        my_game.shuffle_deck();
        my_game.put_cards_onto_board();
    }
    my_game.give_cards_to_players();

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let (s, r) = app::channel::<ChannelMessage>();
    let my_local_ip = network::get_local_ip();
    let tmp_ip = match my_local_ip {
        Some(ip4) => {
            let brd_sender_thrd = thread::spawn(|| {
                thread::sleep(Duration::from_millis(500));
                network::NetworkState::broadcast_ip_and_port(network::BRD_PORT);
            });
            let brd_receiver_thrd = thread::spawn(move || {
                let mut network_state = network::NetworkState::new();
                thread::sleep(Duration::from_millis(1000));
                let sdr = s.clone();
                network_state.receive_broadcast_messages(network::BRD_PORT, sdr);
            });

            ip4.to_string()
        }
        None => String::new(),
    };
    let my_title = format!("Pist Card Game - {} {}", tmp_ip, 0);

    // let my_title = format!("Pisti Card Game - {}", );
    let card_width = 80;
    let card_height = 80;
    let player1_card_top = 300;
    let player2_card_top = 10;
    let mut wind = Window::default()
        .with_label(&my_title.to_owned())
        .with_size(400, 400)
        .center_screen();

    let ai_card1 = Button::new(10, player2_card_top, card_width, card_height, "*");
    let ai_card2 = Button::new(100, player2_card_top, card_width, card_height, "*");
    let ai_card3 = Button::new(190, player2_card_top, card_width, card_height, "*");
    let ai_card4 = Button::new(280, player2_card_top, card_width, card_height, "*");
    let ai_cards: Vec<Button> = vec![ai_card1, ai_card2, ai_card3, ai_card4];

    let card1 = Button::new(10, player1_card_top, card_width, card_height, "Click me!");
    let card2 = Button::new(100, player1_card_top, card_width, card_height, "Click me!");
    let card3 = Button::new(190, player1_card_top, card_width, card_height, "Click me!");
    let card4 = Button::new(280, player1_card_top, card_width, card_height, "Click me!");
    let player_cards: Vec<Button> = vec![card1, card2, card3, card4];

    let mut out1 = Output::new(60, 150, 90, 30, "");
    out1.set_text_size(10);
    out1.set_value("0 P(0)");
    out1.set_label("Player:");
    let mut out2 = Output::new(60, 190, 90, 30, "");
    out2.set_text_size(10);
    out2.set_value("0 P(0)");
    out2.set_label("AI:");

    let board = Button::default()
        .with_pos(10, 10)
        .with_size(card_width, card_height)
        .with_label("Board")
        .center_of(&wind);
    let c_string = format!("{}", my_game.board.last().unwrap());
    board.to_owned().set_label(&c_string);
    board.to_owned().deactivate();

    for my_index in 0..4 {
        let ai_but = ai_cards.get(my_index).unwrap();
        ai_but.to_owned().deactivate();
        let a_string = format!("{}", my_game.player2_hand.get(my_index).unwrap());
        ai_but.to_owned().set_label(&a_string);

        let pl_but = player_cards.get(my_index).unwrap();
        let b_string = format!("{}", my_game.player1_hand.get(my_index).unwrap());
        pl_but.to_owned().set_label(&b_string);
    }

    wind.end();
    wind.show();

    for my_index in 0..4 {
        let pl_but = player_cards.get(my_index).unwrap();
        pl_but.to_owned().emit(
            s,
            ChannelMessage::EM(EventMessage {
                card_index: my_index as u8,
                the_player: Player1,
            }),
        );
    }

    while app.wait() {
        if let Some(c_msg) = r.recv() {
            match c_msg {
                ChannelMessage::EM(msg) => {
                    println!("{:#?}", msg);
                    let a_card = my_game.player1_hand.remove(msg.card_index as usize);
                    println!("you played: {}", a_card);
                    let stat = my_game.play_card(a_card);
                    my_game.move_cards_if_win(stat, Player1);
                    let ai_card_index = my_game.pick_card_for_ai();
                    let a_card = my_game.player2_hand.remove(ai_card_index);
                    println!("ai played: {}", a_card);
                    let stat = my_game.play_card(a_card);
                    my_game.move_cards_if_win(stat, Player2);
                    if my_game.player1_hand.len() < 1 && my_game.player2_hand.len() < 1 {
                        if my_game.deck.len() > 7 {
                            my_game.give_cards_to_players();
                        } else {
                            // last player get all remaining cards on board
                            my_game.move_cards_if_win(Win, my_game.get_last_player());
                        }
                    }
                    update_ui_on_button_press(
                        &ai_cards,
                        &player_cards,
                        &board,
                        &my_game,
                        &out1,
                        &out2,
                    );
                }
                ChannelMessage::NM(a_msg) => {
                    process_network_msg(a_msg, my_local_ip, tmp_ip.to_owned(), &mut wind);
                }
            }
        }
    }
}

mod animation;
mod events;
mod game;
mod network;

use animation::*;
use events::*;
use fltk_flex::Flex;
use fltk_theme::{ThemeType, WidgetTheme};
use std::io::Error;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::{thread, time::Duration};

use fltk::{
    app, button::Button, dialog::input_default, output::Output, prelude::*, window::Window,
};

use game::*;

use crate::network::{send_invite_request, BRD_PORT};

fn main() -> Result<(), Error> {
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

    let app = app::App::default();
    let theme = WidgetTheme::new(ThemeType::Metro);
    theme.apply();
    let (s, r) = app::channel::<ChannelMessage>();
    let my_local_ip = network::get_local_ip();
    let tmp_ip = match my_local_ip {
        Some(ip4) => {
            let _peer_receiver_thrd = thread::spawn(|| {
                network::NetworkState::wait_for_remote_connect(network::BRD_PORT);
            });
            let _brd_sender_thrd = thread::spawn(|| {
                thread::sleep(Duration::from_millis(500));
                network::NetworkState::broadcast_ip_and_port(network::BRD_PORT)
                    .expect("Cannot broadcast message");
            });
            let _brd_receiver_thrd = thread::spawn(move || {
                let mut network_state = network::NetworkState::new();
                thread::sleep(Duration::from_millis(1000));
                let sdr = s.clone();
                network_state
                    .receive_broadcast_messages(network::BRD_PORT, sdr)
                    .expect("Error trying to receive broadcast");
            });

            ip4.to_string()
        }
        None => String::new(),
    };
    let my_title = format!("Pist Card Game - {} {}", tmp_ip, 0);

    let window_width = 600;
    let window_height = 600;
    let mut wind = Window::default()
        .with_label(&my_title.to_owned())
        .with_size(window_width, window_height)
        .center_screen();
    let mut flex = Flex::default()
        .with_size(window_width - 10, window_height - 10)
        .center_of_parent()
        .column();

    let mut upper_flex = Flex::default().row();
    let mut upper2_flex = Flex::default().with_size(200, 100).center_of_parent().row();
    upper2_flex.set_margin(10);
    let ai_card1 = Button::default().with_label("*");
    let ai_card2 = Button::default().with_label("*");
    let ai_card3 = Button::default().with_label("*");
    let ai_card4 = Button::default().with_label("*");
    let ai_cards: Vec<Button> = vec![ai_card1, ai_card2, ai_card3, ai_card4];
    upper2_flex.end();
    upper_flex.end();

    let mut center_flex = Flex::default().row();
    let mut center2_flex = Flex::default().with_size(200, 100).center_of_parent().row();
    center2_flex.set_margin(10);
    center2_flex.end();
    center_flex.end();
    let mut bottom_flex = Flex::default().row();
    let mut bottom2_flex = Flex::default().with_size(200, 100).center_of_parent().row();
    bottom2_flex.set_margin(10);

    let card1 = Button::default().with_label("Click");
    let card2 = Button::default().with_label("Click");
    let card3 = Button::default().with_label("Click");
    let card4 = Button::default().with_label("Click");
    let player_cards: Vec<Button> = vec![card1, card2, card3, card4];
    bottom2_flex.end();
    bottom_flex.end();

    for my_index in 0..4 {
        let ai_but = ai_cards.get(my_index).unwrap();
        ai_but.to_owned().deactivate();
        let a_string = format!("{}", my_game.player2_hand.get(my_index).unwrap());
        ai_but.to_owned().set_label(&a_string);
        set_button_color(&ai_but);

        let pl_but = player_cards.get(my_index).unwrap();
        let b_string = format!("{}", my_game.player1_hand.get(my_index).unwrap());
        pl_but.to_owned().set_label(&b_string);
        set_button_color(&pl_but);
    }


    flex.end(); // end of widgets managed by flex


    let board = Button::default()
        .with_label("Click")
        .with_size(
            player_cards.get(0).unwrap().width(),
            player_cards.get(0).unwrap().height(),
        )
        .center_of_parent();
    let c_string = format!("{}", my_game.board.last().unwrap());
    board.to_owned().set_label(&c_string);
    board.to_owned().deactivate();
    set_button_color(&board);

    let mut open_dialog = Button::default()
        .with_size(150, 20)
        .with_label("Multiplayer")
        .center_of(&wind);
    open_dialog.set_pos(open_dialog.x() + 150, open_dialog.y());
    open_dialog.set_callback(move |_widg| match input_default("Input a ip address", "") {
        Some(a_str) => match Ipv4Addr::from_str(&a_str) {
            Ok(an_ip4) => s.send(ChannelMessage::Dialog(an_ip4)),
            Err(_) => {}
        },
        None => {}
    });

    let mut out1 = Output::new(60, 150, 0, 0, "");
    out1.set_text_size(10);
    out1.set_value("0 P(0)");
    // out1.set_label("Player:");
    let mut out2 = Output::new(60, 190, 0, 0, "");
    out2.set_text_size(10);
    out2.set_value("0 P(0)");
    // out2.set_label("AI:");
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
                ChannelMessage::Dialog(an_ip4) => {
                    thread::spawn(move || {
                        let r = send_invite_request(my_local_ip.unwrap(), an_ip4, BRD_PORT);
                        match r {
                            Ok(_) => {}
                            Err(e) => {
                                println!("error invite: {}", e)
                            }
                        }
                    });
                }
            }
        }
    }
    Ok(())
}

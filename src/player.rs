extern crate futures;
extern crate tokio_core;

use std::io;
use std::str::FromStr;

use tarpc::future::{client, server};
use tarpc::util::FirstSocketAddr;
use tarpc::Error as TarpcError;
use tarpc::future::client::ClientExt;
use self::futures::Future;
use server::FutureServiceExt;
use self::tokio_core::reactor;

use super::server::{PlayerServer, FutureClient};
use super::types::{Action, HintType, Suite};

pub trait Player {
    fn get_command(&mut self, player_id: usize) -> Action;
}

fn read_command() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match String::from_str(input.as_str().trim()) {
                Ok(n) => n,
                Err(error) => panic!("Failed to read from io::stdin with: {}", error),
            }
        }
        Err(error) => panic!("Failed to read from io::stdin with: {}", error),
    }
}

// TODO find lasting generic solution
fn name_suite_map(name: char) -> Suite {
    match name {
        'w' => Suite::White,
        'y' => Suite::Yellow,
        'g' => Suite::Green,
        'b' => Suite::Blue,
        'r' => Suite::Red,
        _ => panic!("Unknown color passed to name_suite_map"),
    }
}

fn get_command_line(player_id: usize) -> Action {
    println!("Choose an action: (p)lay a card, (d)iscard a card, or (h)int a player.");
    match read_command().as_str() {
        "p" => {
            println!("Enter the index of the card you want to play.");
            let index = read_command().parse::<usize>().unwrap();
            Action::Play { index: index }
        }
        "d" => {
            println!("Enter the index of the card you want to discard.");
            let index = read_command().parse::<usize>().unwrap();
            Action::Discard { index: index }
        }
        "h" => {
            println!("Enter the ID of the player you want to hint.");
            let id = read_command().parse::<usize>().unwrap();
            if id == player_id {
                panic!("You can't hint yourself!")
            }
            println!("What type of hint do you want to give?");
            println!("Enter (c)olor or (n)umber.");
            match read_command().as_str() {
                "c" => {
                    println!("Enter the color you want to hint:");
                    println!("(w)hite, (y)ellow, (g)reen, (r)ed, (b)lue");
                    let suite = name_suite_map(read_command().as_str().chars().nth(0).unwrap());
                    Action::Hint {
                        receiver_id: id,
                        hint: HintType::SuiteType(suite),
                    }
                }
                "n" => {
                    println!("Enter the number you want to hint:");
                    let index = read_command().parse::<usize>().unwrap();
                    Action::Hint {
                        receiver_id: id,
                        hint: HintType::Number(index),
                    }
                }
                _ => panic!("Received invalid input"),
            }
        }
        _ => panic!("Received invalid input"),
    }
}

pub struct CommandLinePlayer;

impl Player for CommandLinePlayer {
    fn get_command(&mut self, player_id: usize) -> Action {
        get_command_line(player_id)
    }
}

// TODO: Observe the state
pub struct NaiveAIPlayer;

impl Player for NaiveAIPlayer {
    fn get_command(&mut self, player_id: usize) -> Action {
        // TODO
        Action::Play { index: 0 }
    }
}

pub struct NetworkPlayer {
    // NetworkPlayer represents the local client's connection to another player (an async publisher)
    // metadata for connecting to server
    // Format is <ip>:port
    // Default port: 7864
    address: String,
    // port: String,
    // player_server: PlayerServer,
    server_handle: server::Handle,
    reactor: reactor::Core,
    // client_handle: Connect<FutureClient>,
}

impl Player for NetworkPlayer {
    fn get_command(&mut self, player_id: usize) -> Action {
        // Block until we get a command.
        // TODO Connect client and cache connections!
        let options = client::Options::default().handle(self.reactor.handle());
        self.reactor
            .run(FutureClient::connect(self.server_handle.addr(), options)
                     .map_err(TarpcError::from)
                     .and_then(|client| client.get_action(player_id)))
            .unwrap()
    }
}

impl NetworkPlayer {
    pub fn new(address: String) -> NetworkPlayer {
        let mut reactor = reactor::Core::new().unwrap();
        let (server_handle, server) = PlayerServer { last_action: None }
            .listen(address.first_socket_addr(),
                    &reactor.handle(),
                    server::Options::default())
            .unwrap();
        reactor.handle().spawn(server);

        // let options = client::Options::default().handle(reactor.handle());
        // let client_handle = FutureClient::connect(server_handle.addr(), options);
        NetworkPlayer {
            address,
            server_handle,
            reactor,
             // client_handle,
        }
    }
}

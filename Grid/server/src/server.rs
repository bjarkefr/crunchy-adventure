extern crate ws;
extern crate env_logger;
extern crate uuid;

use std::collections::HashMap;
use self::uuid::Uuid;
use self::ws::listen;

use world::World;

pub fn run (addr: &str) {

    let mut game_registry = HashMap::<String, World>::new();

    //env_logger::init().unwrap();

    println!("Listening for connections...");

    if let Err(error) = listen(addr, |out| {

        println!("Got a new connection got dammit!");

        let game_id = Uuid::new_v4().simple().to_string();

        let mut game_world = game_registry.insert(game_id.clone(), World::new()).unwrap();
        {
            let player = game_world.new_player();

            println!("New game id: {}, {}", game_id, player.get_id());
        }

        let view = game_world.render_for_player(); //player

        out.send(view).unwrap();

        move |msg| {

            println!("Server got message '{}'. ", msg);

            out.send(msg)
        }
    }) {
        println!("Failed to create WebSocket due to {:?}", error);
    }
}

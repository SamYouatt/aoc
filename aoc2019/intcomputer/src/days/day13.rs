use std::{collections::HashMap, sync::mpsc::{self, Receiver, Sender}, thread};

use crate::{
    computer::{parse_tape, Computer},
    Point,
};

enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

pub fn part1(input: &str) -> usize {
    let tape = parse_tape(input);
    let (computer_sender, computer_receiver) = mpsc::channel();
    let (arcade_sender, arcade_receiver) = mpsc::channel();

    let mut grid: HashMap<Point, Tile> = HashMap::new();
    let mut computer = Computer::load(&tape, computer_receiver, arcade_sender);

    // Hoping this will close the robot's sender and so it will know to stop
    thread::spawn(move || {
        computer.run();
        drop(computer);
    });

    arcade(arcade_receiver, &mut grid);

    grid.values().filter(|t| matches!(*t, &Tile::Block)).count()
}

fn arcade(receiver: Receiver<i64>, grid: &mut HashMap<Point, Tile>) {
    // Loop until broken
    // If no first instruction then break - computer has halted -> x coord
    // Grab second instruction as well -> y coord
    // Grab third instruction as well -> tile type

    loop {
        let Ok(x_instruct) = receiver.recv() else {
            break;
        };
        let y_instruct = receiver.recv().expect("computer shouldn't close mid cycle");
        let tile_instruct = receiver.recv().expect("computer shouldn't close mid cycle");

        let position = Point::new(x_instruct, y_instruct);
        let tile = match tile_instruct {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!("unknown tile instruction {}", tile_instruct),
        };

        grid.insert(position, tile); 
    }
}

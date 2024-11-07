use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::{
    computer::{parse_tape, Computer},
    Point,
};

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

enum Event {
    TileUpdate(Point, Tile),
    ScoreUpdate(usize),
}

pub fn part1(input: &str) -> usize {
    let tape = parse_tape(input);
    let (_computer_sender, computer_receiver) = mpsc::channel();
    let (arcade_sender, arcade_receiver) = mpsc::channel();

    let mut grid: HashMap<Point, Tile> = HashMap::new();
    let mut computer = Computer::load(&tape, computer_receiver, arcade_sender);

    thread::spawn(move || {
        computer.run();
        drop(computer);
    });

    arcade(arcade_receiver, &mut grid);

    grid.values().filter(|t| matches!(*t, &Tile::Block)).count()
}

pub fn part2(input: &str) -> usize {
    let mut tape = parse_tape(input);
    tape[0] = 2; // Put quarter in to play

    let (computer_sender, computer_receiver) = mpsc::channel();
    let (output_processor_sender, output_processor_receiver) = mpsc::channel();
    let (event_sender, event_receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, computer_receiver, output_processor_sender);

    let mut grid: HashMap<Point, Tile> = HashMap::new();
    let mut score = 0;

    thread::spawn(move || {
        computer.run();
        drop(computer);
    });

    thread::spawn(move || {
        output_processor(event_sender, output_processor_receiver);
    });

    // From debugging worked out that the computer expects input immediately
    computer_sender.send(0).unwrap();

    loop {
        let Ok(event) = event_receiver.recv() else {
            break;
        };

        match event {
            Event::ScoreUpdate(new_score) => {
                score = new_score;
            }
            Event::TileUpdate(point, tile) => {
                grid.insert(point, tile);

                let Some(ball_pos) = grid
                    .iter()
                    .find(|(_k, v)| matches!(v, &&Tile::Ball))
                    .map(|(point, _tile)| point)
                else {
                    continue; // ball not loaded yet or game finished
                };
                let Some(paddle_pos) = grid
                    .iter()
                    .find(|(_k, v)| matches!(v, &&Tile::HorizontalPaddle))
                    .map(|(point, _tile)| point)
                else {
                    continue; // paddle not loaded yet or game finished
                };

                // Only perform input on ball update to avoid filling up queue with same input
                if !matches!(tile, Tile::Ball) {
                    continue;
                }

                let joystick_input = match ball_pos.x.cmp(&paddle_pos.x) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                };

                let _ = computer_sender.send(joystick_input); // Computer may have ended by this point but we don't care
            }
        }
    }

    score
}

fn output_processor(event_sender: Sender<Event>, processor_receiver: Receiver<i64>) {
    loop {
        let Ok(x) = processor_receiver.recv() else {
            break;
        };
        let y = processor_receiver
            .recv()
            .expect("computer closed mid instruction");
        let z = processor_receiver
            .recv()
            .expect("computer closed mid instruction");

        let event = match (x, y, z) {
            (-1, 0, score) => Event::ScoreUpdate(score as usize),
            (x, y, z) => Event::TileUpdate(Point::new(x, y), parse_tile(z)),
        };

        event_sender
            .send(event)
            .expect("game loop should not close before event processor");
    }
}

fn arcade(receiver: Receiver<i64>, grid: &mut HashMap<Point, Tile>) {
    loop {
        let Ok(x_instruct) = receiver.recv() else {
            break;
        };
        let y_instruct = receiver.recv().expect("computer shouldn't close mid cycle");
        let tile_instruct = receiver.recv().expect("computer shouldn't close mid cycle");

        let position = Point::new(x_instruct, y_instruct);
        let tile = parse_tile(tile_instruct);

        grid.insert(position, tile);
    }
}

fn parse_tile(instruction: i64) -> Tile {
    match instruction {
        0 => Tile::Empty,
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::HorizontalPaddle,
        4 => Tile::Ball,
        _ => panic!("unknown tile instruction {}", instruction),
    }
}

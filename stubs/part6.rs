// Part 6: Files, traits and serialisation

// add functionality for ./starroom --file <json_file>

// for those who finish early, some further ideas:

// - support multiple files
// - load a JSON file over HTTP
// - if any parsing fails, replace with a random value

extern crate rand;
extern crate docopt;
extern crate rustc_serialize;

use std::fs::File;
use std::io::prelude::*;

use rustc_serialize::json;

use docopt::Docopt;

const USAGE: &'static str = "
The Star Room!

Usage:
    starroom [<length> <width> <stars>...]
    starroom --file <json_file>

Options:
    -h --help             Show this screen.
    --file <json_file>    JSON file with star data
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_length: Option<u8>,
    arg_width: Option<u8>,
    arg_stars: Vec<u8>,
    flag_file: Option<String>
}

#[derive(RustcDecodable)]
struct Point {
    x: u8,
    y: u8
}

#[derive(RustcDecodable)]
struct Room {
    length: u8,
    width: u8,
    stars: Vec<Point>
}

impl Room {
    fn new(length: u8, width: u8) -> Room {
        Room {
            length: length,
            width: width,
            stars: vec!()
        }
    }

    fn add_star(&mut self, new_star: Point) -> Option<()> {
        if new_star.x == 0 || new_star.x == self.length-1 ||
            new_star.y == 0 || new_star.y == self.width-1 {
            return None;
        }
        for star in &self.stars {
            if new_star.x == star.x && new_star.y == star.y {
                return None;
            }
        }

        self.stars.push(new_star);
        Some(())
    }

    // randomly decide where to place the star
    // stars can only be drawn inside the room, which has an area of
    // length-2, width-2, then push it out of wall range with +1
    fn add_random_star(&mut self) {
        let new_star = Point {
            x: (rand::random::<u8>() % (self.length-2)) + 1,
            y: (rand::random::<u8>() % (self.width-2)) + 1
        };

        self.stars.push(new_star);
    }

    fn is_star_at_point(&self, current_point: &Point) -> bool {
        for star in &self.stars {
            if star.x == current_point.x && star.y == current_point.y {
                return true;
            }
        }

        return false;
    }

    fn point_is_wall(&self, point: &Point) -> bool {
        point.y == 0 || point.y == self.width-1
            || point.x == 0 || point.x == self.length-1
    }

    fn draw(&self) {
        let star = "🌟";

        // for each line (down)
        for y in 0..self.width {
            // for each character (across)
            for x in 0..self.length {
                // if we are drawing a wall
                let current_point = Point{x: x, y: y};
                if self.point_is_wall(&current_point) {
                    print!("=");
                } else if self.is_star_at_point(&current_point) {
                    print!("{}", star);
                } else { // if we aren't drawing a wall
                    print!(" ");
                }
            }
            print!("\n");
        }
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("Welcome to the star room");
    println!("Debug: {:?}", args);

    if args.flag_file.is_some() {
        // FILL THIS IN
    }

    let mut room = Room::new(args.arg_length.unwrap_or(20),
                             args.arg_width.unwrap_or(8));

    if args.arg_stars.len() == 0 {
        room.add_random_star();
    } else if args.arg_stars.len() % 2 == 1 {
        panic!("Must provide an even number of star coords!");
    } else {
        // Make sure we have an even number of values
        let mut i = 0;
        while i < args.arg_stars.len() {
            room.add_star(Point{x: args.arg_stars[i],
                                y: args.arg_stars[i+1]});
            i += 2;
        }
    }

    room.draw();
}

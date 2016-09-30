// Part 5: argument parsing and error handling

// add functionality for ./starroom <length> <width> <star_x> <star_y>

extern crate rand;

use std::env;

struct Point {
    x: u8,
    y: u8
}

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
    let args: Vec<String> = env::args().collect();

    println!("Welcome to the star room");

    // if we weren't given any arguments
    if args.len()-1 == 0 {
        let mut room = Room::new(20, 8);
        room.add_random_star();
        room.draw();
    // if we were given appropriate arguments
    } else if args.len()-1 >= 4 && (args.len()-1) % 2 == 0 {
        let mut room = Room::new(args[1].parse::<u8>().unwrap(),
                                 args[2].parse::<u8>().unwrap());
        for i in 3..args.len() {
            if i % 2 == 0 {
                continue;
            }
            room.add_star(Point{x: args[i].parse::<u8>().unwrap(),
                                y: args[i+1].parse::<u8>().unwrap()})
                .unwrap();
        }
        room.draw();
    } else {
        println!("Invalid arguments, use no args or <l> <w> <starx> <stary>");
    }
}

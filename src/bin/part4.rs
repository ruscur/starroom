// Part 4: structs

// Use structs to make our program more awesome.

extern crate rand;

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

    fn add_star(&self, new_star: Point) {
        if new_star.x == 0 || new_star.x == self.length-1 ||
            new_star.y == 0 || new_star.y == self.width-1 {
            return;
        }
        for star in self.stars {
            if new_star.x == star.x && new_star.y == star.y {
                return
            }
        }

        self.stars.push(new_star);
    }

    // randomly decide where to place the star
    // stars can only be drawn inside the room, which has an area of
    // length-2, width-2, then push it out of wall range with +1
    fn add_random_star(&self) {
        let new_star = Point {
            x: (rand::random::<u8>() % (self.length-2)) + 1,
            y: (rand::random::<u8>() % (self.width-2)) + 1
        };

        self.stars.push(new_star);
    }
}

fn point_is_wall(room: Room, point: Point) -> bool {
    point.y == 0 || point.y == room.width-1 || point.x == 0 || point.x == room.length-1

    // mention this ^ is the same as if that { return true } else { return false }
}

fn main() {
    println!("Welcome to the star room");

    let star = "🌟";

    let room = Room::new(20, 8);
    room.add_random_star();

    // for each line (down)
    for y in 0..room.width {
        // for each character (across)
        for x in 0..room.length {
            // if we are drawing a wall
            if point_is_wall(room, Point{x: x, y: y}) {
                print!("=");
            } else { // if we aren't drawing a wall
                print!(" ");
            }
        }
        print!("\n");
    }
}

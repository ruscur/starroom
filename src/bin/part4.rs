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
    stars: mut Vec<Point>
}

impl Room {
    fn new(length: u8, width: u8) {
        Room {
            length: length,
            width: width,
            stars: vec!()
        }
    }
}

fn point_is_wall(room: Room, point: Point) -> bool {
    point.y == 0 || point.y == room.width-1 || point.x == 0 || point.x == room.length-1

    // mention this ^ is the same as if that { return true } else { return false }
}

fn main() {
    println!("Welcome to the star room");

    let star = "🌟";
    let room = Room {
        length = 20,
        width = 8,
        stars = 1
    };

    // randomly decide where to place the star
    // stars can only be drawn inside the room, which has an area of
    // length-2, width-2, then push it out of wall range with +1
    let star_x = (rand::random::<u8>() % (length-2)) + 1;
    let star_y = (rand::random::<u8>() % (width-2)) + 1;

    // for each line (down)
    for y in 0..width {
        // for each character (across)
        for x in 0..length {
            // if we are drawing a wall
            if point_is_wall(length, width, x, y) {
                print!("=");
            } else if x == star_x && y == star_y {
                print!("{}", star);
            } else { // if we aren't drawing a wall
                print!(" ");
            }
        }
        print!("\n");
    }
}

// Part 4: structs, borrowing and mutability

extern crate rand;

struct Point {
    x: u8,
    y: u8
}

struct Room {
    // FILL THIS IN
}

impl Room {
    fn new(length: u8, width: u8) -> Room {
        // FILL THIS IN
    }

    fn new_with_stars(length: u8, width: u8, stars: Vec<Point>) -> Room {
        // FILL THIS IN
    }

    fn add_star(&mut self, new_star: Point) {
        // FILL THIS IN
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
    println!("Welcome to the star room");

    let mut room = Room::new(20, 8);
    room.add_random_star();

    room.draw();
}

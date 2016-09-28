// Part 2: functions and types

// Give an overview of Rust's builtin types and function syntax

fn point_is_wall(length: u8, width: u8, x: u8, y: u8) -> bool {
    y == 0 || y == width-1 || x == 0 || x == length-1

    // mention this ^ is the same as if that { return true } else { return false }
}

fn main() {
    println!("Welcome to the star room");

    let length = 20;
    let width = 8;

    // for each line (down)
    for y in 0..width {
        // for each character (across)
        for x in 0..length {
            // if we are drawing a wall
            if point_is_wall(length, width, x, y) {
                print!("=");
            } else { // if we aren't drawing a wall
                print!(" ");
            }
        }
        print!("\n");
    }
}

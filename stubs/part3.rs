// Part 3: crates

// Add the rand crate to cargo and use it to draw a star.

// Import the rand crate
// FILL THIS IN

fn point_is_wall(length: u8, width: u8, x: u8, y: u8) -> bool {
    y == 0 || y == width-1 || x == 0 || x == length-1
}

fn main() {
    println!("Welcome to the star room");

    let length = 20;
    let width = 8;
    let star = "🌟";

    // randomly decide where to place the star

    // stars can only be drawn inside the room, which has an area of
    // length-2, width-2, then push it out of wall range with +1
    let star_x = // FILL THIS IN
    let star_y = // FILL THIS IN

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

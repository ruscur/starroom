// Part 1: language fundamentals

// create a new project with `cargo new --bin starroom`
// edit Cargo.toml to add some personal metadata

fn main() {
    println!("Welcome to the star room");

    let length = 20;
    let width = 8;

    // for each line (down)
    for y in 0..width {
        // for each character (across)
        for x in 0..length {
            // if we are drawing a wall
            if y == 0 || y == width-1 || x == 0 || x == length-1 {
                print!("=");
            } else { // if we aren't drawing a wall
                print!(" ");
            }
        }
        print!("\n");
    }
}

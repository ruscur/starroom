// Part 2: functions and types

// Give an overview of Rust's builtin types and function syntax

fn point_is_wall(// FILL ME IN // ) -> // ME TOO // {
    // FILL ME IN //
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
            if point_is_wall(// FILL ME IN //) {
                print!("=");
            } else { // if we aren't drawing a wall
                print!(" ");
            }
        }
        print!("\n");
    }
}

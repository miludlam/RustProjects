/**
 * An example program that calculates the area of a rectangle
 * and is refactored into using tuples and refactored again to
 * use structs
 */
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

use std::io;

struct Sides {
    length: f32,
    breadth: f32,
    height: f32,
    base: f32,
}

fn main() {
    println!("Enter the type of shape \n[1.Triangle  2.Rectangle  3.Square]");
    let mut shape_type = String::new();
    io::stdin()
        .read_line(&mut shape_type)
        .expect("Not a valid floating point number, please try again.");

    let sides = Sides {
        length: create_struct("Length"),
        breadth: create_struct("Breadth"),
        height: create_struct("Height"),
        base: create_struct("Base"),
    };

    match shape_type.trim() {
        "1" => area_of_triangle(&sides),
        "2" => area_of_rectangle(&sides),
        "3" => area_of_square(&sides),
        _ => println!("Shut the fuck up, maths is hard. i cant implement more formulas."),
    };
}

fn create_struct(side: &str) -> f32 {
    println!("Enter {}: ", side);
    let mut var = String::new();
    io::stdin()
        .read_line(&mut var)
        .expect("Not a valid floating point number, please try again.");
    return var.trim().parse().expect("lmao");
}
fn area_of_triangle(sides: &Sides) {
    println!("Its a triange");
    println!(
        "The area of this Trianlge is {}",
        (sides.base * sides.height) / 2.0
    );
}
fn area_of_rectangle(sides: &Sides) {
    println!("Its a rectangle");
    println!(
        "The area of this Rectangle is {}",
        sides.length * sides.breadth
    );
}
fn area_of_square(sides: &Sides) {
    println!("Its a square");
    println!("The area of this Square is {}", sides.length * sides.length);
}

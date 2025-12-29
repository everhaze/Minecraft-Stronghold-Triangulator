use std::{io, thread, time::Duration};
use colored::*;

fn main() {
    let mut x1 = String::new();
    let mut x2 = String::new();
    let mut yaw1 = String::new(); //in f3, the first one is yaw.
    let mut z1 = String::new();
    let mut z2 = String::new();
    let mut yaw2 = String::new();

    println!("{}", ">>> End Portal Triangulator <<<".purple());
    println!("Enter your x coords.");
    io::stdin().read_line(&mut x1).expect("Invalid input.");
    println!("Enter your z coords.");
    io::stdin().read_line(&mut z1).expect("Invalid input.");
    println!("Enter the eye's yaw angle.");
    io::stdin().read_line(&mut yaw1).expect("Invalid input.");

    println!("{}", "Turn 90 degrees to the right/left (NOT TOWARDS EYE) and run 200-500 blocks towards that direction.".purple());
    thread::sleep(Duration::from_secs(3));

    println!("Enter your new x coords.");
    io::stdin().read_line(&mut x2).expect("Invalid input.");
    println!("Enter your new z coords.");
    io::stdin().read_line(&mut z2).expect("Invalid input.");
    println!("Enter the eye's yaw angle.");
    io::stdin().read_line(&mut yaw2).expect("Invalid input.");

    let x1i: f32 = x1.trim().parse().unwrap();
    let x2i: f32 = x2.trim().parse().unwrap();
    let yaw1i: f32 = yaw1.trim().parse().unwrap();
    let z1i: f32 = z1.trim().parse().unwrap();
    let z2i: f32 = z2.trim().parse().unwrap();
    let yaw2i: f32 = yaw2.trim().parse().unwrap();

    triangulator(x1i, x2i, z1i, z2i, yaw1i, yaw2i);

}

fn triangulator(x1: f32, x2: f32, z1: f32, z2: f32, yaw1: f32, yaw2: f32) {
    let yaw1_rad = (yaw1 + 90.0).to_radians();
    let yaw2_rad = (yaw2 + 90.0).to_radians();

    let m1 = yaw1_rad.tan();
    let m2 = yaw2_rad.tan();

    let denominator = m1 - m2;

    let x = (z2 - z1 + m1 * x1 - m2 * x2) / denominator;
    let z = m1 * (x - x1) + z1;

    println!("{}", ">>> >>> >>> >> << <<< <<< <<<".purple());
    println!("Eyes crossing at x = {}, z = {}", x, z);
}
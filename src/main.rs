//F[n] = F[n-1] + F[n-2]
use std::io;

fn main() {
    let mut length_of_sequence = String::new();
    println!("Enter length of sequence");
    io::stdin().read_line(&mut length_of_sequence)
        .expect("Failed to read line");
    let length_of_sequence: u32 = length_of_sequence.trim().parse()
        .expect("Pleace type a number!");
//    if length_of_sequence == 1 {println!("1: 0");}
//    if length_of_sequence == 2 {println!}
    let mut f: u128;     //F[n]
    let mut f1: u128 = 0;    //F[n-1]
    let mut f2: u128 = 0;    //F[n-2]
    let mut i = 1;
    while i <= length_of_sequence {
        if i == 1 {
            f = 0;
            f1 = f;
        } else
        if i == 2 {
            f = 1;
            f2 = f1;
            f1 = f;
        } else {
            f = f1 + f2;
            f2 = f1;
            f1 = f;
        }
        println!("{}: {}", i, f);
        i = i + 1;
    }
}

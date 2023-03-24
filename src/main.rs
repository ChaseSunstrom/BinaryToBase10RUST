use std::io;

fn main() {
    loop {
        println!("Enter a binary number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let i = i64::from_str_radix(input, 2).expect("Not a binary number!");
        println!("{i}");

        println!("Break? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input = input.trim().to_lowercase();

        if input.contains('y'){
            break;
        } else{
            continue;
        }
    }
}

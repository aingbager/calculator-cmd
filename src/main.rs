mod function1;
use std::io::{self, Write};

fn main() {
    let mut user_input1 = String::new();
    print!("masukan angka: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input1).unwrap();

    let mut user_input2 = String::new();
    print!("masukan angka: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input2).unwrap();

    let user_input1: i32 = user_input1.trim().parse().expect("inputkan angka");
    let user_input2: i32 = user_input2.trim().parse().expect("inputkan angka");

    let mut pilihan = String::new();
    print!("pilih operasi [*/+-]: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut pilihan).unwrap();

    let pilih = pilihan.trim().chars().next();

    let result = match pilih {
        Some('*') => function1::kali(user_input1, user_input2),
        Some('/') => function1::bagi(user_input1, user_input2),
        Some('+') => function1::tambah(user_input1, user_input2),
        Some('-') => function1::kurang(user_input1, user_input2),
        _ => {
            println!("inputan salah");
            return;
        }
    };

    println!("hasil: {}", result);
}

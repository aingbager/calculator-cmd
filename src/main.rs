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
        Some('*') => kali(user_input1, user_input2),
        Some('/') => bagi(user_input1, user_input2),
        Some('+') => tambah(user_input1, user_input2),
        Some('-') => kurang(user_input1, user_input2),
        _ => {
            println!("inputan salah");
            return;
        }
    };

    println!("hasil: {}", result);
}

fn kali(a: i32, b: i32) -> i32 {
    let result = a * b;
    return result;
}

fn bagi(a: i32, b: i32) -> i32 {
    if b == 0 {
        println!("pembagian salah");
        std::process::exit(1);
    }
    a / b
}

fn tambah(a: i32, b: i32) -> i32 {
    let result = a + b;
    return result;
}

fn kurang(a: i32, b: i32) -> i32 {
    let result = a / b;
    return result;
}

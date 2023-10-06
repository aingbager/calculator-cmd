pub fn kali(a: i32, b: i32) -> i32 {
    let result = a * b;
    return result;
}

pub fn bagi(a: i32, b: i32) -> i32 {
    if b == 0 {
        println!("pembagian salah");
        std::process::exit(1);
    }
    a / b
}

pub fn tambah(a: i32, b: i32) -> i32 {
    let result = a + b;
    return result;
}

pub fn kurang(a: i32, b: i32) -> i32 {
    let result = a / b;
    return result;
}

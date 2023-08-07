use std::io;



fn main() {

    let mut a: String = "".to_string();
    println!("=============================");
    println!("Введите строку");
    io::stdin().read_line(&mut a);
    println!("Вы ввели: {:?}",a);
    a = "".to_string();
    println!("=============================");
    println!("Введите число");
    io::stdin().read_line(&mut a);
    println!("Вы ввели: {:?}",a);
    let b : i32 = a.trim().parse().unwrap();
    println!("Пытаемся прочитать b: {:?}",b);
// Test change
}

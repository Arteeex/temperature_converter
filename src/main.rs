use  std::io;

fn main() {
    
    println!("1. Перевести градусы Цельсия в Фаренгейт");
    println!("2. Перевести градусы Фаренгейта в Цельсий");


    let mut degree = String::new();
    let mut number = String::new();



    io::stdin()
        .read_line(&mut number)
        .expect("");
    let number: i32 = number
        .trim()
        .parse()
        .expect("");

    println!("Введите температуру: ");

    io::stdin()
        .read_line(&mut degree)
        .expect("");

    let degree: f64 = degree
        .trim()
        .parse()
        .expect("");

    if number == 1 { 
        println!("{c}", c = (degree * 1.8) + 32.0);
    } else if number == 2 {
        println!("{f:.2}", f = (degree - 32.0) * 5.0/9.0 );
    } else {
        println!("Ввели некорректное значение");
    } 

}
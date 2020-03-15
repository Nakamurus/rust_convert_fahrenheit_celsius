use std::io;

fn main() {
    let mut temp = String::new();

    let mut value = String::new();

    println!(
        "If you want to convert fahrenheit to celsius, type f, if celsius to fahrenheit, type c"
    );

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: String = temp.trim().to_string();

    println!("Type temperature");

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: f64 = value.trim().parse().unwrap();

    translate_fahrenheit_celsius(temp, value)
}

fn translate_fahrenheit_celsius(temp: String, val: f64) {
    // 浮動小数点型にしないと、小数点以下が出ると0を返す
    if temp == "f" {
        let temperature: f64 = (5.0 / 9.0) * (val - 32.0);
        println!("Celsius temperature is: {}", temperature);
    } else if temp == "c" {
        let temperature: f64 = (9.0 / 5.0) * (val + 32.0);
        println!("Fahrenheit temperature is: {}", temperature);
    } else {
        println!("type f if  Fahrenheit -> Celsius, else type c");
        main()
    }
}

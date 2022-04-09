use std::io;

fn main() {
    let mut x: f64 = loop {
        print!("Enter a number: ");
        io::Write::flush(&mut io::stdout()).expect("Failed to flush");
        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Failed to read line");
        match x.trim().parse() {
            Ok(x) => break x,
            Err(_) => {
                println!("Invalid input. Try again.");
                continue;
            }
        }
    };

    'outer: loop {
        let sign: String = loop {
            print!("Enter a sign: ");
            io::Write::flush(&mut io::stdout()).expect("Failed to flush");
            let mut x = String::new();
            io::stdin().read_line(&mut x).expect("Failed to read line");
            match x.trim() {
                "+" | "-" | "/" | "*" => break x,
                _ => {
                    println!("Invalid input. Try again.");
                    continue;
                }
            }
        };

        let y: f64 = loop {
            print!("Enter a number: ");
            io::Write::flush(&mut io::stdout()).expect("Failed to flush");
            let mut x = String::new();
            io::stdin().read_line(&mut x).expect("Failed to read line");
            match x.trim().parse() {
                Ok(x) => break x,
                Err(_) => {
                    println!("Invalid input. Try again.");
                    continue;
                }
            }
        };

        println!("{} {} {} = {}", x, &sign.trim(), y, calculate(x, &sign, y));
        x = calculate(x, &sign, y);

        let answer: String = loop {
            print!("Continue? [Y/n] ");
            io::Write::flush(&mut io::stdout()).expect("Failed to flush");
            let mut x = String::new();
            io::stdin().read_line(&mut x).expect("Failed to read line");
            break x;
        };

        if answer.trim() == "n" || answer.trim() == "N" {
            println!("{}", x);
            break 'outer;
        }
    }
}

fn calculate(x: f64, sign: &String, y: f64) -> f64 {
    match sign.trim() {
        "-" => x - y,
        "+" => x + y,
        "*" => x * y,
        "/" => x / y,
        _ => 0.0,
    }
}

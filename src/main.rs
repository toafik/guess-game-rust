use rand::RngExt;
use std::io;
fn difficulty() -> u32 {
    println!("zvol si obtížnost:");
    println!("Hint: vyber si maximální číslo do kterého se může generovat randomizace");
    let mut x: String = String::new();
    io::stdin().read_line(&mut x).expect("Error");
    let x: u32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("špatná hodnota, zkus používat čísla ty vajíčko <3");
            return difficulty(); // zkusí to znovu od začátku
        }
    };
    x
}

fn main() {
    let max = difficulty();
    let mut pokusy = 0;
    let number = rand::rng().random_range(1..=max);
    println!("zkus trefit random čislo!");
    println!("*--------------------------------------------------------------*");

    loop {
        println!("zadej svuj guess:");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Error"); // <- tohle chybělo
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("špatná hodnota, zkus používat čísla ty vajíčko <3");
                println!("*--------------------------------------------------------------*");
                continue;
            }
        };
        println!("Tvoje čislo je {guess}");
        println!("*--------------------------------------------------------------*");
        if guess == number {
            println!("spravně, to číslo bylo {number}");
            break;
        } else {
            println!("špatně!");
            if guess > number {
                println!("máš moc velké čislo!");
                pokusy += 1;
                println!("máš za sebou už {pokusy}");
                println!("*--------------------------------------------------------------*");
            } else {
                println!("máš moc malé čislo!");
                pokusy += 1;
                println!("máš za sebou už {pokusy}");
                println!("*--------------------------------------------------------------*");
            }
        }
    }
}

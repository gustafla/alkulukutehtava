use std::io::{self, BufRead, Write};
use std::str::FromStr;

fn kysy_luku<T: FromStr>(kysymys: &str) -> T {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        print!("{}: ", kysymys);
        stdout.flush().unwrap();
        stdin.lock().read_line(&mut input).unwrap();
        match input.trim().parse::<T>() {
            Ok(val) => return val,
            Err(_) => println!("Virheellinen syöte, yritä uudelleen"),
        }
        input.clear();
    }
}

fn main() {
    let alaraja: usize = kysy_luku("Anna alueen alaraja");
    let ylaraja: usize = kysy_luku("Anna alueen yläraja");

    let mut alkuluvut = vec![true; ylaraja + 1];
    alkuluvut[1] = false;

    for i in (2usize..).take_while(|x| x * x <= ylaraja) {
        if alkuluvut[i] {
            for j in (0..).map(|x| i * i + x * i).take_while(|x| x <= &ylaraja) {
                alkuluvut[j] = false;
            }
        }
    }

    for i in alaraja..ylaraja {
        print!("{} ", i);
        if alkuluvut[i] {
            println!("on alkuluku.");
        } else {
            println!("ei ole alkuluku.");
        }
    }

    println!("Kiitos ohjelman käytöstä.");
}

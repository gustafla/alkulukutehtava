use std::io::{self, BufRead, Write};
use std::str::FromStr;
use std::collections::HashMap;

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

    // TODO could possibly use only a map
    let mut alkuluvut = vec![true; ylaraja + 1];
    let mut tekijat = HashMap::with_capacity(ylaraja / 20);

    for i in (2usize..).take_while(|x| x * x <= ylaraja) {
        if alkuluvut[i] {
                for j in (0..).map(|x| i * i + x * i).take_while(|x| x <= &ylaraja) {
                alkuluvut[j] = false;
                tekijat.insert(j, (i, j/i));
            }
        }
    }

    for i in alaraja..ylaraja+1 {
        print!("{} ", i);
        if alkuluvut[i] {
            println!("on alkuluku.");
        } else {
            if i < 2 {
                println!("ei ole kelvollinen alkuluku.");
            } else {
                let (n, m) = tekijat.get(&i).unwrap();
                if cfg!(debug_assertions) {
                    assert_eq!(n * m, i);
                    print!("Assertion passed");
                }
                println!("ei ole alkuluku, koska {} * {} = {}", n, m, i);
            }
        }
    }

    println!("Kiitos ohjelman käytöstä.");
}

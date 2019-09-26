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
    let mut tekijat = vec![(0, 0); ylaraja + 1];
    alkuluvut[1] = false;

    for i in (2usize..).take_while(|x| x * x <= ylaraja) {
        if alkuluvut[i] {
            for (j, m) in (0..).map(|x| (i * i + x * i, x)).take_while(|(x, _)| x <= &ylaraja) {
                alkuluvut[j] = false;
                tekijat[j] = (i, m); //?
            }
        }
    }

    for i in alaraja..ylaraja {
        print!("{} ", i);
        if alkuluvut[i] {
            println!("on alkuluku.");
        } else {
            if i == 1 {
                println!("ei ole kelvollinen alkuluku.");
            } else {
                let (n, m) = tekijat[i];
                println!("ei ole alkuluku, koska {} * {} = {}", n, m, i);
            }
        }
    }

    println!("Kiitos ohjelman käytöstä.");
}

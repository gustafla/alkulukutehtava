use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

fn kysy<T: FromStr>(kysymys: &str) -> T {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        print!("{}: ", kysymys);
        stdout.flush().unwrap();
        stdin.lock().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(val) => return val,
            Err(_) => println!("Virheellinen syöte, yritä uudelleen"),
        }
        input.clear();
    }
}

fn main() {
    let alaraja: usize = kysy("Anna alueen alaraja");
    let ylaraja: usize = kysy("Anna alueen yläraja");

    let mut alkuluvut = vec![true; ylaraja + 1];
    let mut tekijat = HashMap::with_capacity(ylaraja / 20);

    for i in (2usize..).take_while(|x| x * x <= ylaraja) {
        if alkuluvut[i] {
            for j in (0..).map(|x| i * i + x * i).take_while(|x| x <= &ylaraja) {
                alkuluvut[j] = false;
                tekijat.insert(j, (i, j / i));
            }
        }
    }

    let mut viimeinen = 0;
    for i in alaraja..ylaraja + 1 {
        print!("{} ", i);
        if i < 2 {
            println!("ei ole kelvollinen alkuluku.");
        } else if alkuluvut[i] {
            println!("on alkuluku.");
            viimeinen = i;
        } else {
            let (n, m) = tekijat.get(&i).unwrap();
            println!("ei ole alkuluku, koska {} * {} = {}", n, m, i);
        }
    }

    if viimeinen != 0 {
        println!("Viimeinen löydetty alkuluku oli {}", viimeinen);
    }

    println!("Kiitos ohjelman käytöstä.");
}

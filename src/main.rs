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

    let mut tekija = vec![0; ylaraja + 1];

    for i in (2usize..).take_while(|x| x * x <= ylaraja) {
        if tekija[i] == 0 {
            for j in (0..).map(|x| i * i + x * i).take_while(|x| x <= &ylaraja) {
                tekija[j] = i;
            }
        }
    }

    let mut viimeinen = 0;
    for i in alaraja..ylaraja + 1 {
        print!("{} ", i);
        if i < 2 {
            println!("ei ole kelvollinen alkuluku.");
        } else if tekija[i] == 0 {
            println!("on alkuluku.");
            viimeinen = i;
        } else {
            println!("ei ole alkuluku, koska {} * {} = {}", tekija[i], i / tekija[i], i);
        }
    }

    if viimeinen != 0 {
        println!("Viimeinen löydetty alkuluku oli {}", viimeinen);
    }

    println!("Kiitos ohjelman käytöstä.");
}

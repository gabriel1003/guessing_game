use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Bem-vindo ao nosso jogo de adivinações!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Digite o seu paupite: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("O seu palpite foi: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("O número secreto é maior!"),
            Ordering::Greater => println!("O número secreto é menor!"),
            Ordering::Equal => {
                println!("Você descobrio o número secreto!");
                break;
            }
        }
    }
}

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("O número secreto é: {secret_number}");
    loop {
        println!("Digite o seu palpite: ");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("Você digitou: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Você ganhou!");
break;
            }
        }
    }
}

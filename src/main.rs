use std::io;
use rand::Rng;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("O número secreto é: {secret_number}");
    
    println!("Digite o seu palpite: ");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess).expect("Failed to read line");
    println!("Você adivinhou: {}", guess);
}

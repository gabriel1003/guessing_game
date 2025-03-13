use std::io;

fn main() {
    println!("Adivinhe o número!");

    println!("Digite o seu palpite: ");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess).expect("Failed to read line");
    println!("Você adivinhou: {}", guess);
}

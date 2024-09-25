use std::io::stdin;

fn main() {
    println!("Digite uma string: ");

    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);

    let num = buf.chars().filter(|&c| c == 'a' || c == 'A').count();

    match num {
        0 => println!("Não há nenhuma letra 'A' na string passada!"),
        _ => println!("Número de 'A's encontrados: {num}"),
    }
}

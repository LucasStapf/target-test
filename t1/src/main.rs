use std::{io::stdin, u128};

fn main() {
    println!("Digite um número: ");

    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    let num = buf
        .trim()
        .parse::<u128>()
        .expect("Não foi possível converter a entrada para um número!");

    match is_fib(num) {
        true => println!("O número {num} pertence à sequência!"),
        false => println!("O número {num} NÃO pertence à sequência!"),
    }
}

fn is_fib(n: u128) -> bool {
    let mut last: (u128, u128) = (0, 1);

    loop {
        let new: u128 = last
            .0
            .checked_add(last.1)
            .expect("Não foi possível calcular o próximo termo da sequência: Overflow");

        if new == n {
            return true;
        } else if new > n {
            return false;
        }

        last.0 = last.1;
        last.1 = new;
    }
}

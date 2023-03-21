use std::io::{self, Write};
use std::io::{stdin, Read};
use std::process::Command;

fn main() {
    println!("Kaprekar");
    loop {
        //Command::new("clear").status().unwrap();
        let mut kpkcifras = [0; 4];
        let mut i = 0;
        let mut kpkcompletado = true;

        let mut kpk_formado = 0;
        let mut kpk_formado_des = 0;
        let mut neokpk = 0;
        let mut kpk_input = String::new();
        let mut steps = 0;

        print!("Introduce un número de 4 cifras, donde todas no pueden ser iguales: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut kpk_input)
            .expect("Error al leer la entrada");

        let mut kpk_input = match kpk_input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Se introdujo algo que no era un número :)\n");
                continue;
            }
        };

        while kpkcompletado {           //"\x1b[31mHello, World!\x1b[0m" //"\x1b[32mEl número nuevo formado es --------- \x1b[0m"
            i = 0;
            kpk_formado = 0;
            kpk_formado_des = 0;
            kpkcifras = [0; 4];

            if kpk_input == 1111 || kpk_input == 2222 || kpk_input == 3333 || kpk_input == 4444 || kpk_input == 5555 || kpk_input == 6666 || kpk_input == 7777 || kpk_input == 8888 || kpk_input == 9999 || kpk_input <= 999 {
                println!("Estás poniendo cosas que no te dije\n");
                break;
            }

            let mut kpk_input_copia = kpk_input;

            while kpk_input_copia > 0 {
                kpkcifras[i] = kpk_input_copia % 10;
                kpk_input_copia /= 10;
                i += 1;
            }

            // Ascendente acá
            kpkcifras.sort();

            for (d, kpkcifra) in kpkcifras.iter().enumerate() {
                kpk_formado += kpkcifra * 10_i32.pow((kpkcifras.len() - 1 - d) as u32);
            }

            kpkcifras.reverse();

            for (d, kpkcifra) in kpkcifras.iter().enumerate() {
                kpk_formado_des += kpkcifra * 10_i32.pow((kpkcifras.len() - 1 - d) as u32);
            }

            println!("\x1b[31mEl número elegido ordenado a menor es {}\x1b[0m", kpk_formado);
            println!("\x1b[34mEl número elegido ordenado a mayor es {}\x1b[0m", kpk_formado_des);

            neokpk = kpk_formado_des - kpk_formado;
            println!("\x1b[32mEl número nuevo formado es --------- {}\x1b[0m", neokpk);

            kpk_input = neokpk;
            steps += 1;

            println!("Continuar...");
            io::stdin().read_line(&mut String::new())
                .expect("Error al leer la entrada");

            if neokpk == 6174 {
                kpkcompletado = false;
            }
        }
        println!("Se ha completado tu kaprekar con {} pasos ✨✨", steps);
        read_key();
        break;
    }
}
fn read_key() -> char {
    let mut buffer = [0; 1];
    stdin().read(&mut buffer).expect("Error al leer la entrada");
    buffer[0] as char
}

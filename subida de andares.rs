use std::io;

fn possibilidade(mut andares: i32) -> f64{

    let mut possibilidades = 0.0;

    if andares <= 3 {
        
        match andares {
            3 => possibilidades = 4.0,
            2 => possibilidades = 2.0,
            1 => possibilidades = 1.0,
            _ => println!("numeor  invalido")
        }
        
    }else{
        andares -= 3;
        possibilidades = f64::powi(4.0, andares);
        possibilidades *= 4.0;
    }
    
    possibilidades
}

fn main() {

    println!("Digite o numero  de andares a subir: ");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    let mut entrada: i32 = entrada.trim().parse().unwrap();

    
    println!("Possibilidades = {}",possibilidade(entrada));

}

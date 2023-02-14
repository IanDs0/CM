use std::io;

fn possibilidade(mut andares: i32) -> i32{

    let mut possibilidades = 0;

    if andares <= 3 {
        
        match andares {
            3 => possibilidades = 6,
            2 => possibilidades = 3,
            1 => possibilidades = 10,
            _ => println!("numeor  invalido")
        }
        
    }else{
        andares -= 3;
        possibilidades = 6 + (andares*3);
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

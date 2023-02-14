use std::io;

fn inverter_diagonais(matriz: &mut Vec<Vec<i32>>) {
    let n = matriz.len();
    let mut aux: i32 = 0;
    for i in 0..n {
        let j = n-1-i;
        aux = matriz[i][i];
        matriz[i][i] = matriz[i][j];
        matriz[i][j] = aux;
    }
}

fn main() {

    println!("Digite o tamanho da matriz quadrada:");
    let mut tamanho = String::new();
    io::stdin().read_line(&mut tamanho).unwrap();
    let tamanho: i32 = tamanho.trim().parse().unwrap();

    let mut matriz = vec![vec![0; tamanho as usize]; tamanho as usize];

    println!("Digite os elementos da matriz:");
    for i in 0..tamanho as usize {
        for j in 0..tamanho as usize {
            let mut elemento = String::new();
            io::stdin().read_line(&mut elemento).unwrap();
            let elemento: i32 = elemento.trim().parse().unwrap();
            matriz[i][j] = elemento;
        }
    }

    println!("Matriz original: {:?}", matriz);
    inverter_diagonais(&mut matriz);
    println!("Matriz invertida: {:?}", matriz);
}

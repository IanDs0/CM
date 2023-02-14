use std::io;

fn buscar_padrao(imagem: &Vec<Vec<i32>>, padrao: &Vec<Vec<i32>>) -> bool {
    let m = imagem.len();
    let n = imagem[0].len();
    let m1 = padrao.len();
    let n1 = padrao[0].len();
    
    for i in 0..m-m1+1 {
        for j in 0..n-n1+1 {
            let mut encontrado = true;
            for k in 0..m1 {
                for l in 0..n1 {
                    if imagem[i+k][j+l] != padrao[k][l] {
                        encontrado = false;
                        break;
                    }
                }
                if !encontrado {
                    break;
                }
            }
            if encontrado {
                return true;
            }
        }
    }
    return false;
}

fn main() {

    println!("Digite o tamanho da matriz quadrada da imagem x:");
    let mut tamanho = String::new();
    io::stdin().read_line(&mut tamanho).unwrap();
    let tamanho: i32 = tamanho.trim().parse().unwrap();
    
    println!("Digite o tamanho da matriz quadrada da imagem y:");
    let mut tamanho2 = String::new();
    io::stdin().read_line(&mut tamanho2).unwrap();
    let tamanho2: i32 = tamanho2.trim().parse().unwrap();

    let mut imagem = vec![vec![0; tamanho2 as usize]; tamanho as usize];

    println!("Digite os elementos da matriz:");
    for i in 0..tamanho as usize {
        for j in 0..tamanho2 as usize {
            let mut elemento = String::new();
            io::stdin().read_line(&mut elemento).unwrap();
            let elemento: i32 = elemento.trim().parse().unwrap();
            imagem[i][j] = elemento;
        }
    }
    
    let mut padrao = vec![vec![0; 3 as usize]; 3 as usize];
    
    println!("Digite os elementos da matriz do padrao:");
    for i in 0..3 {
        for j in 0..3 {
            let mut elemento = String::new();
            io::stdin().read_line(&mut elemento).unwrap();
            let elemento: i32 = elemento.trim().parse().unwrap();
            padrao[i][j] = elemento;
        }
    }

    println!("Padrão encontrado na imagem: {}", buscar_padrao(&imagem, &padrao));
}
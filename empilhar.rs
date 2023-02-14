use std::collections::VecDeque;

fn remove(pilha: &mut VecDeque<i32>, x: i32) {
    let mut temp_stack = VecDeque::new();

    while let Some(top) = pilha.pop_back() {
        if top != x {
            temp_stack.push_back(top);
        } else {
            break;
        }
    }

    while let Some(top) = temp_stack.pop_back() {
        pilha.push_back(top);
    }
}

fn main() {
    let mut pilha = VecDeque::new();
    pilha.push_back(1);
    pilha.push_back(2);
    pilha.push_back(3);
    pilha.push_back(4);
    pilha.push_back(5);

    println!("Pilha original: {:?}", pilha);

    remove(&mut pilha, 4);

    println!("Pilha sem o elemento 3: {:?}", pilha);
}

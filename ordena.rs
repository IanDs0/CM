fn main() {

    let mut array = [3,10,4,5,2,9,1,0,8,7];
    let mut aux = 9;
    
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
    
    println!("{:?}",array);

}   
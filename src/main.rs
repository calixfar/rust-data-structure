use std::vec;

fn main() {
    let mut  vector = vec![5, 1, 4, 2, 8];

    println!("unsorted: {:?}", vector);

    bubble_sort(&mut vector);

    println!("sorted: {:?}", vector);
}

fn bubble_sort(vector: &mut Vec<usize>) {
    let mut max_items = vector.len();
    let mut sorted = false;

    while !sorted {
        sorted = true;

        for index in 0..max_items - 1 {
            if vector[index] > vector[index + 1] {
                vector.swap(index, index + 1);
                sorted = false;
            }
        }
        
        max_items -= 1;
    }
}
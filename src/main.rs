fn main() {
    let mut vector = vec![9,1,8,7,6,5,4,3,2];
    selection_sort(&mut vector);
}

fn selection_sort(vector: &mut Vec<usize>) -> Vec<usize> {
    let mut sorted_vector: Vec<usize> = Vec::new();

    while !vector.is_empty() {
        let mut smallest_index = 0;

        for index in 1..vector.len() {
            if vector[index] < vector[smallest_index] {
                smallest_index = index;
            }
        }

        sorted_vector.push(vector[smallest_index]);
        vector.remove(smallest_index);
    }

    println!("{:?}", sorted_vector);

    sorted_vector
}

fn binary_search(vector: &mut Vec<usize>, item_to_search: usize) {
    let mut start: usize = 0;
    let mut end: usize = vector.len();
    let mut index: isize = -1;

    while start < end {
        let middle = (start + end) / 2; 
        // println!("start: {}, end: {}, middle: {}, middle value: {}, splitted{:?}", start, end, middle, vector[middle], &vector[start..end]);
        
        if vector[middle] == item_to_search {
            index = middle as isize;
            
            break;
        } else if vector[middle] > item_to_search {
            end = middle;
        } else {
            start = middle;
        }
    }

    println!("{}", index);
}

fn bubble_sort(vector: &mut Vec<usize>) {
    let mut max_items = vector.len();
    let mut sorted = false;

    while !sorted {
        sorted = true;

        for index in 0..max_items - 1 {
            if vector[index] > vector[index + 1] {
                let temp = vector[index + 1];
                vector[index + 1] = vector[index];
                vector[index] = temp;

                // vector.swap(index, index + 1);
                sorted = false;
            }
        }
        
        max_items -= 1;
    }
}
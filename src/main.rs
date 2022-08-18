fn main() {
    let mut  vector = vec![1,2,3,4,5,6,7,8];

    binary_search(&mut vector, 4);
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
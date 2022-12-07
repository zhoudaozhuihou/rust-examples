fn main() {
    let mut ve1 = vec![5, -3, 0, 3, 1, 2, -1, 8, 7];
    println!("排序前: {:?}",ve1);
    bubble_sort(&mut ve1);
    println!("排序后：{:?}",ve1);
}

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}



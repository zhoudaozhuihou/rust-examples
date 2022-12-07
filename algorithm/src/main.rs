fn main() {
    // let mut v = vec![3, 2, 5, 1, 4];
    let mut v = vec!['a', 'b', 'h', 'e', 'c'];
    println!("before sort: v = {:?}", v);

    bubble_sort(&mut v);
    println!("after sort: v = {:?}", v);
}


#[allow(dead_code)]
fn bubble_sort<T: PartialOrd>(v: &mut Vec<T>) {
    let mut swapped = false;
    for i in 0..(v.len() - 1) {
        if v[i] > v[i + 1] {
            v.swap(i, i + 1);
            swapped = true;
        }
    }
    if swapped {
        bubble_sort(v);
    }
}
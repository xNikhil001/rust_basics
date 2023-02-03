use std::time::Instant;

fn main() {
    let mut arr: [u64;10] = [1, 20000, 6, 445, 0,3444,1123,8888,9999,5];
    // let mut i = 0;
    let now = Instant::now();

    bubble_sort(&mut arr);
    linear_search(arr, 6);
    println!("{:?}", arr);

    // while i < 100000 {
    //     println!("{i}");
    //     i+=1;
    // }

    let elapsed_time = now.elapsed();
    println!(
        "Program took {} nanos {} millis {} seconds.",
        elapsed_time.as_nanos(),elapsed_time.as_millis(),elapsed_time.as_secs()
    );
}

fn bubble_sort(arr: &mut [u64;10]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1)
            }
        }
    }
}

fn linear_search(arr: [u64;10], element: u64) {
    let mut i = 0;

    while i < arr.len() {
        if arr[i] == element {
            return println!("Element: {element} found at index {i}");
        }

        i += 1;
    }
    println!("el not found");
}

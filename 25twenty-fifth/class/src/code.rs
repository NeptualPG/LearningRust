// Function to find the largest in a slice
fn largest_int(list: &[i32]) -> i32 {
    let mut largest = &list[0];
    for item in list.iter(){
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Function to find the largest character in a slice
fn largest_char(list:&[char]) -> &char {
    let mut largest = &list[0];
    for item in liest.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main () {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_int(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}


// Sumary: This code defines two functions, `largest_int` and `largest_char`, which find the largest integer and character in a slice, respectively. The `main` function demonstrates their usage by creating a vector of integers and a vector of characters, calling the respective functions, and printing the results.


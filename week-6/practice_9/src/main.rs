//The iter() function fetches values of all elements in an array.

fn main() {
    let arr : [i32;4] = [10,20,30,40];
    println!("Array is: {:?}",arr);
    println!("Array size :{}",arr.len());

    for val in arr.iter(){
        println!("value is : {}",val );
    }
}

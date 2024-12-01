fn main() {
    //Array with data type(explicit integer data type)
    let arr1 : [i32;4] = [10,20,30,40];
    println!("\nArray with data type");
    println!("Array is: {:?}",arr1 );
    println!("Array size :{}",arr1.len());

    //Array without dataype(implicit float data type)
    let arr2 = [10.4,20.7,30.4,40.9,51.2,72.2];
    println!("\nArray without data type");
    println!("Array is: {:?}",arr2 );
    println!("Array size :{}",arr2.len());

    //Array with default values that create and 
    //initializes all its elements with a default value of 1
    let arr3 : [i32;8] = [-1;8];
    println!("\nArray without data values");
    println!("Array is: {:?}",arr3 );
    println!("Array size :{}",arr3.len());
}

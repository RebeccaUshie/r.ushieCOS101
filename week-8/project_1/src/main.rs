use std::fs::OpenOptions;
use std::io::Write;
fn main() {
    //Defining the categories of the brands
    let lager = vec!["33 export","Desperados","Golder","Heineken","Star"];
    let stout = vec!["Legend","Turbo King","Williams"];
    let non_alcoholic = vec!["Maltina","Amstel Malta","Malta Gold","Fayrouz"];

    let mut file = std::fs::File::create("drinks.txt").expect("create failed");

    // Writing the data in the drinks file
    file.write_all("lager:\n".as_bytes()).unwrap();
    file.write_all(lager.join(",").as_bytes()).unwrap();
    file.write_all("stout:\n".as_bytes()).unwrap();
    file.write_all(stout.join(",").as_bytes()).unwrap();
    file.write_all("non_alcoholic:\n".as_bytes()).unwrap();
    file.write_all(non_alcoholic.join(",").as_bytes()).unwrap();

    println!("The drinks and their categories have been saved into the file.");



    
}

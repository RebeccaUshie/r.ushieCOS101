fn main() {
    // Separating the categories as arrays
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Display merged data
    println!("S/N | Name of Commissioner | Ministry | Geopolitical Zone");
    for i in 0..commissioners.len() {
        println!(
            "{}   | {} | {} | {}",
            i + 1,
            commissioners[i],
            ministries[i],
            geopolitical_zones[i]
        );
    }
}

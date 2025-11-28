fn main() {
    // Dataset 1: Names of Commissioners
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    // Dataset 2: Ministries
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // Dataset 3: Geopolitical Zones
    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    println!("S/N | Commissioner Name                | Ministry         | Geopolitical Zone");
    println!("----------------------------------------------------------------------------");

    for i in 0..commissioners.len() {
        println!(
            "{}   | {:30} | {:15} | {}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        );
    }
}

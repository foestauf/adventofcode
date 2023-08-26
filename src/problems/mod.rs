mod year_2022;

pub fn select_year(number: &str) {
    match number {
        "2022" => year_2022::year_2022(),
        _ => println!("Invalid problem number"),
    }
}

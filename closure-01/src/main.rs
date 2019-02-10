use std::fmt::Debug;

#[derive(Debug)]
struct City {
    Name: String,
    Population: i64,
    Country: String,
}

impl City {
    fn new(name: String, population: i64, country: String) -> Self {
        City {
            Name: name.to_string(),
            Population: population,
            Country: country.to_string(),
        }
    }
}

fn population_descending(city: &City) -> i64 {
    -city.Population
}

fn sort(cities: &mut Vec<City>) {
        cities.sort_by_key(population_descending);
}

//fn sort_cities(cities: &mut Vec<City>) {
    //??
//}

fn main() {
    let mut cities = vec![
        City::new("Tokyo".to_string(), 1000000, "Japan".to_string()),
        City::new("Nara".to_string(), 1000, "Kyoto".to_string()),
        City::new("Nanba".to_string(), 10000, "Osaka".to_string()),
        ];
    println!("{:?}", cities);
    sort(&mut cities);
    println!("{:?}", cities);
}

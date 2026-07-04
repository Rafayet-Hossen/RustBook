const MAX_CARGO_WEIGHT: u32 = 10_000;

fn main() {
    let mut current_weight = 8500;
    println!("Current Weight is {current_weight}");
    current_weight += 1200;
    println!("Updated Weight is {current_weight}");

    let fuel_gauge = "75%";
    let fuel_gauge: u32 = fuel_gauge.trim_matches('%').parse().unwrap();

    println!("The Fuel value {fuel_gauge}");

    let sector_id: char = 'A';

    let hyperdrive_active = true;

    let engine_efficiency: f32 = 0.98;

    let space_coordinates: (i32, f64, char) = (-45, 128.54321, 'X');

    let lat = space_coordinates.0;
    let lon = space_coordinates.1;
    let sec = space_coordinates.2;

    println!("Longitude: {lon} Sector {sec}");

    let mut thruster_statuses = [true; 4];

    thruster_statuses[1] = false;

    println!("{:?}", thruster_statuses);
}

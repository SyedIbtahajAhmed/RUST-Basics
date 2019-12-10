mod vehicle;
fn main() {
    let car_01 = vehicle::vehicle_prop::asso_prop(
        String::from("Corolla"),
        String::from("Toyota"),
        "White".to_string(),
        1300,
        2019,
    );
    println!("{:?}", vehicle::vehicle_prop::Vehicle::val_print(&car_01));
}

pub mod vehicle_prop{
#[derive(Debug)]
pub struct Vehicle{
    name: String,
    company_n: String,
    color: String,
    transmission: i32,
    model: i32,
}
    pub fn asso_prop(n: String, comp_n: String, col: String, trans: i32, mo: i32)-> Vehicle{
        Vehicle{
        name: n,
        company_n: comp_n,
        color: col,
        transmission: trans,
        model: mo,
        }
    }
impl Vehicle{
    pub fn val_print(&self){
        println!("Name: {}", self.name);
        println!("Company Name: {}", self.company_n);
        println!("Color: {}", self.color);
        println!("Transmission: {}", self.transmission);
        println!("Model: {}", self.model);
    }
}
}
fn get_occupation(name: &str)->Option<&str>{
    return match name{
        "Louis" => Some("dev"),
        _ => None
    }
}
fn main() {
    println!("occupation is {}", match get_occupation("Freddy"){

        Some(o) => o,
        None => "no occupation found",
    });


}
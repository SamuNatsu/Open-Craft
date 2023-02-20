// Define modules
mod graphic;
mod resource;

fn main() {
    let settings = resource::settings::Settings::read("settings.toml").unwrap();
    println!("{:?}", settings);
}

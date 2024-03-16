use rand::Rng;
use time::Weekday;

fn main() {
    let mut rng = rand::thread_rng();
    let random_weekday: Weekday = rng.gen();
    println!("How about we conduct Fix The Bug on {:?}?", random_weekday);
}

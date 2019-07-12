extern crate tam_tam_bot;

fn main() {
    let bot = tam_tam_bot::TamTam::new(String::from("z6QUEUsazV-8ic8eXpmBfB5cVrMB3Reo6Owj4tU9BGQ"));
    println!("{:#?}", bot.get_info().unwrap().text());
}

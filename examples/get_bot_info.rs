use tam_tam_bot;

fn main() -> Result<(), std::io::Error> {
    let bot = tam_tam_bot::TamTam::new(String::from("z6QUEUsazV-8ic8eXpmBfB5cVrMB3Reo6Owj4tU9BGQ"));

    let json = bot.get_info();
    println!("{:#?}", json);
    Ok(())
}

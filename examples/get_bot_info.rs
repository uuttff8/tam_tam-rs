use tam_tam_bot::{bot::TTBot, tam_tam::TamTam};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = TamTam::new(String::from("z6QUEUsazV-8ic8eXpmBfB5cVrMB3Reo6Owj4tU9BGQ"));
    let bot: TTBot = bot.get_bot_info()?;
    println!("{:#?}", bot);
    Ok(())
}

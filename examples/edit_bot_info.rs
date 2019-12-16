use tam_tam_bot::tam_tam::{bot::TTBot, TamTam};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = TamTam::new(String::from("z6QUEUsazV-8ic8eXpmBfB5cVrMB3Reo6Owj4tU9BGQ"));

    let mut edited_bot: TTBot = bot.get_bot_info()?;
    edited_bot.description = Some("Rewrite TamTam in Rust!".into());

    let result = bot.edit_bot_info(edited_bot);
    println!("{:#?}", result);
    Ok(())
}

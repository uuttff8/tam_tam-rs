use tam_tam_bot::tam_tam::ChatJson;

fn main() -> reqwest::Result<()> {
    let bot = tam_tam_bot::TamTam::new(String::from("z6QUEUsazV-8ic8eXpmBfB5cVrMB3Reo6Owj4tU9BGQ"));

    let json: ChatJson = bot.get_all_chats()?.json()?;
    println!("{:#?}", json.chats[0].status);
    // Ok(())
    // println!("{:#?}", bot.get_members(15330663644)?.text()?);
    //println!("{:#?}", bot.get_chat_by_id(15330663644)?.text()?);
    Ok(())
}

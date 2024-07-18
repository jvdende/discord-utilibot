use crate::gameslist;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::Data, Error>;

/// Wanna play ping pong?
#[poise::command(slash_command, ephemeral = true)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("Pong!");
    ctx.reply(response).await?;
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    rename = "gameslist",
    guild_only = true,
    subcommands("gameslist_command_show"),
    subcommand_required
)]
pub async fn gameslist_command(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Show games on the list
#[poise::command(prefix_command, slash_command, rename = "show")]
pub async fn gameslist_command_show(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().to_string();

    //if let Some(id) = ctx.guild_id() {
    //    gameslist::create_file(id.to_string());
    //}

    let mut gameslist = gameslist::gameslist(guild_id);
    gameslist.sort_by_key(|game| game.name.to_lowercase());

    let mut response = String::new();

    for game in gameslist {
        let response_string = format_show_entry_line(game.name, game.steam_uri);
        response.push_str(&response_string);
    }

    ctx.say(response).await?;
    Ok(())
}

fn format_show_entry_line(name: String, steam_uri: Option<String>) -> String {
    let mut str = format!("* {name}");

    if let Some(uri) = steam_uri {
        let steam_uri_format = format!(" - [_Steam_]({uri})");
        str.push_str(&steam_uri_format);
    }

    format!("{}\n", str)
}

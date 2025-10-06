use std::env;

use poise::serenity_prelude::{
    CacheHttp, Client, ClientBuilder, GatewayIntents, GuildId
};
use poise::{
    Framework, FrameworkOptions
};
use songbird::SerenityInit;

use crate::discord_bot_rs::commands::util::ping;
use crate::discord_bot_rs::commands::voice::join;
use crate::prelude::{Data, PoiseError, PoiseResult};

pub async fn create_bot(intents: GatewayIntents, token: String) -> PoiseResult<Client> {
    let framework = Framework::builder()
        .options(FrameworkOptions::<Data, PoiseError> {
            // To be populated soon
            commands: vec![ping(), join()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework|{
            Box::pin(async move {
                let env_gid: u64 = env::var("DISCORD_GUILD")?.parse::<u64>()?;
                let guild_id: GuildId = GuildId::new(env_gid);
                
                poise::builtins::register_in_guild(
                    ctx.http(), 
                    &framework.options().commands, 
                    guild_id
                ).await?;

                println!("Successfully loaded {} commands", framework.options().commands.len());
                Ok(Data {})
            })
        })
        .build();

    let client: Client = ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await?;

    Ok(client)
}
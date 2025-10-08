use poise::serenity_prelude::Mentionable;
use songbird::{CoreEvent};

use crate::discord_bot_rs::audio::handler::VoiceHandler;
use crate::prelude::{PoiseContext, PoiseResult};
use crate::discord_bot_rs::commands::cmd;
use crate::discord_bot_rs::utility::send_reply_eph;


/// Join the voice channel of the user
#[cmd(slash_command, guild_only)]
pub async fn join(ctx: PoiseContext<'_>) -> PoiseResult<()> {
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Failed to get Songbird manager from context")?;

    let guild = ctx
        .guild().ok_or("Failed to get Guild informations, check permissions!")?
        .clone();

    let user_voice_state = guild
        .voice_states.get(&ctx.author().id);

    let voice_channel = match user_voice_state {
        Some(uvs) => {
            match uvs.channel_id {
                Some(chid) => chid.to_channel(ctx.http()).await?,
                None => {
                    send_reply_eph(ctx.clone(), "You must be in a voice channel to use this command!").await?;
                    return Ok(());
                }
            }
        }
        None => {
            send_reply_eph(ctx.clone(), "You must be in a voice channel to use this command!").await?;
            return Ok(());
        }
    };

    match manager.join(guild.id, voice_channel.id()).await {
        Ok(_mutex_call) => {
            let text: String = format!("Successfully join channel {}", voice_channel.mention().to_string());
            send_reply_eph(ctx.clone(), text).await?;

            let mut call = _mutex_call.lock().await;
            let voice_handler = VoiceHandler::new();

            call.add_global_event(CoreEvent::ClientDisconnect.into(), voice_handler.clone());
            call.add_global_event(CoreEvent::SpeakingStateUpdate.into(), voice_handler.clone());
            call.add_global_event(CoreEvent::DriverConnect.into(), voice_handler.clone());
            call.add_global_event(CoreEvent::DriverDisconnect.into(), voice_handler.clone());
            call.add_global_event(CoreEvent::DriverReconnect.into(), voice_handler.clone());
            call.add_global_event(CoreEvent::VoiceTick.into(), voice_handler.clone());
        }
        Err(e) => return Err(Box::new(e))
    }

    Ok(())
}


/// Leave any voice channels the bot is in
#[cmd(slash_command, guild_only)]
pub async fn leave(ctx: PoiseContext<'_>) -> PoiseResult<()> {
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Failed to get Songbird manager from context")?;

    let guild = ctx
        .guild().ok_or("Failed to get Guild informations, check permissions!")?
        .clone();

    if guild.voice_states.get(&ctx.author().id).is_none() {
        let text = "You can't disconnect the bot if you are not in a voice channel!";
        send_reply_eph(ctx.clone(), text).await?;
    } else {
        manager.leave(guild.id).await?;
        send_reply_eph(ctx, "Successfully disconnected bot from voice channels").await?;
    }

    Ok(())
}
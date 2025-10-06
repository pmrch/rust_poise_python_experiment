use crate::discord_bot_rs::commands::cmd;
use crate::discord_bot_rs::utility::send_reply_eph;
use crate::prelude::{PoiseContext, PoiseResult};


/// Returns the bot latency in ms or ns
#[cmd(slash_command, guild_only)]
pub async fn ping(ctx: PoiseContext<'_>) -> PoiseResult<()> {
    let text: String = format!("The bot latency is {:?}", ctx.ping().await);
    send_reply_eph(ctx, text).await?;
    Ok(())
}
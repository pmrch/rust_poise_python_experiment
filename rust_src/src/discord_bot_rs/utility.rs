use poise::CreateReply;

use crate::prelude::{PoiseContext, PoiseResult};

pub async fn send_reply_eph<T: ToString>(ctx: PoiseContext<'_>, text: T) -> PoiseResult<()> {
    let rep: CreateReply = CreateReply {
        content: Some(text.to_string()),
        ephemeral: Some(true),
        ..Default::default()
    };

    ctx.send(rep).await?;
    Ok(())
}
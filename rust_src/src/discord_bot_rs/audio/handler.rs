use poise::serenity_prelude::async_trait;
use songbird::{
    Event, EventContext, EventHandler
};

use crate::logging::{log_async};


#[derive(Clone, Copy)]
pub struct VoiceHandler {}


#[async_trait]
impl EventHandler for VoiceHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        let _ = match ctx {
            EventContext::ClientDisconnect(cd) => {
                let msg = format!("User disconnected with ID: {}", cd.user_id.0);
                log_async(log::Level::Info, msg).await;
            }
            EventContext::SpeakingStateUpdate(ssu) => {
                let msg = format!("Speaking State Updated for SSRC {}", ssu.ssrc);
                log_async(log::Level::Info, msg).await;
            }
            EventContext::DriverConnect(dc) => {
                let msg = format!("Bot successfully joined voice channel with SSRC {}", dc.ssrc);
                log_async(log::Level::Info, msg).await;
            }
            EventContext::DriverDisconnect(ddc) => {
                let msg = format!("Bot disconnected from voice channel with session id {}", ddc.session_id);
                log_async(log::Level::Warn, msg).await;
            }
            EventContext::DriverReconnect(dr) => {
                let msg = format!("Bot successfully reconnected with SSRC {:?}", dr.ssrc);
                log_async(log::Level::Info, msg).await;
            }
            EventContext::VoiceTick(vt) => {
                println!("REAL AUDIO! {:?}", vt.speaking);
            }
            _ => return None
        };

        None
    }
}
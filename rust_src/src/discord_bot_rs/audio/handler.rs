use std::sync::Arc;

use poise::serenity_prelude::{async_trait, futures::lock::Mutex};
use songbird::{
    Event, EventContext, EventHandler
};

use crate::{discord_bot_rs::audio::{decoder::decode_opus_packet, manager::MultiUserAudio}, logging::log_async};


#[derive(Clone)]
pub struct VoiceHandler {
    m_u_audio: Arc<Mutex<MultiUserAudio>>
}

#[async_trait]
impl EventHandler for VoiceHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        let _ = match ctx {
            EventContext::ClientDisconnect(cd) => {
                let msg = format!("User disconnected with ID: {}", cd.user_id.0);
                log_async(log::Level::Info, msg).await;
            }
            EventContext::SpeakingStateUpdate(ssu) => {
                {
                    let mua = self.m_u_audio.lock().await;
                    mua.create_user_queue(ssu.ssrc).await; 
                }

                let msg = format!("Speaking State Updated for SSRC {}", ssu.ssrc);
                log_async(log::Level::Info, msg).await;

                if ssu.speaking.microphone() {
                    
                }
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
                let speaking = vt.speaking.clone();

                let mut mua = self.m_u_audio.lock().await;
                for item in speaking {
                    if mua. item.0
                }
            }
            _ => return None
        };

        None
    }
}

impl VoiceHandler {
    pub fn new() -> Self {
        Self { 
            m_u_audio: Arc::new(Mutex::new(MultiUserAudio::new()))
        }
    }
}
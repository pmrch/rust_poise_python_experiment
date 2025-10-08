use std::{collections::{HashMap, VecDeque}, sync::Arc};

use poise::serenity_prelude::futures::lock::Mutex;

use crate::{logging::log_sync, prelude::PoiseResult};


#[derive(Clone, Debug)]
pub struct MultiUserAudio {
    // The map itself is locked only for insertion/removal/clear
    audio_queue: Arc<Mutex<HashMap<u32, Arc<Mutex<VecDeque<i16>>>>>>,

    // Shared list of SSRCs, locked when modified
    stored_ssrcs: Arc<Mutex<Vec<u32>>>,

    // Cache last ssrc
    current_ssrc: Arc<Mutex<u32>>
}

impl MultiUserAudio {
    pub fn new() -> Self {
        Self { 
            audio_queue: Arc::new(Mutex::new(HashMap::new())), 
            stored_ssrcs: Arc::new(Mutex::new(Vec::new())),
            current_ssrc: Arc::new(Mutex::new(0))
        }
    }

    pub async fn create_user_queue(&self, user_ssrc: u32) {
        *self.current_ssrc.lock().await = user_ssrc;
        let mut map_lock = self.audio_queue.lock().await;

        // Insert a new queue only if it doesn't exist yet
        map_lock.entry(user_ssrc)
            .or_insert_with(|| Arc::new(Mutex::new(VecDeque::new())));
    }

    pub async fn get_user_ssrcs(&self) -> Vec<u32> {
        let map = self.audio_queue.lock().await;
        map
            .keys()
            .map(|u| *u)
            .collect::<Vec<u32>>()
    }

    pub async fn push(&self, buf: Vec<i16>, user_ssrc: u32) -> PoiseResult<()> {
        *self.current_ssrc.lock().await = user_ssrc;
        let user_queue_arc = {
            let map_lock = self.audio_queue.lock().await;

            match map_lock.get(&user_ssrc) {
                Some(q) => q.clone(),
                None => {
                    log_sync(log::Level::Error, format!("Failed to get user buffer linked to SSRC {}", &user_ssrc));
                    return Err("Failed to get user buffer from SSRC".into());
                }
            }
        };

        user_queue_arc
            .lock()
            .await
            .extend(buf);

        Ok(())
    }
}

pub struct UserAudio {

}
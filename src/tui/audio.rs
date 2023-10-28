use anyhow::Result;
use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use std::io::Cursor;

// TODO add support for multiple sound
pub struct SoundBank {
    audio_mngr: AudioManager,
    sound_data: StaticSoundData,
}

impl SoundBank {
    pub fn from_array(buf: &[u8]) -> Result<Self> {
        let data_cursor = Cursor::new(buf.to_vec());
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
        let sound_data = StaticSoundData::from_cursor(data_cursor, StaticSoundSettings::default())?;
        Ok(Self {
            audio_mngr: manager,
            sound_data,
        })
    }

    pub fn play(&mut self) -> Result<()> {
        self.audio_mngr.play(self.sound_data.clone())?;

        Ok(())
    }
}

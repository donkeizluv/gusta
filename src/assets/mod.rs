pub fn alert_sound() -> &'static [u8] {
    include_bytes!("embed/alert.wav")
}

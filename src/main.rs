use std::error::Error;
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", ".wav");
    audio.add("lose", ".wav");
    audio.add("move", ".wav");
    audio.add("pew", ".wav");
    audio.add("startup", ".wav");
    audio.add("win", ".wav");
    audio.play("startup");
    //Cleanup
    audio.wait();
    Ok(())
}

use std::{sync::OnceLock, thread::sleep, time::{Duration, Instant}};

fn main() {
    let settings = Settings::get_setting();
    println!("{}", settings.time.elapsed().as_secs());
    sleep(Duration::from_secs(1));

    let settings2 = Settings::get_setting();
    sleep(Duration::from_secs(1));
    println!("{:?}",settings2.time == settings.time);

    let settings3 = Settings::get_setting();
    sleep(Duration::from_secs(1));
    println!("{:?}", settings3.time == settings.time);

    let settings4 = Settings::get_setting();
    sleep(Duration::from_secs(1));
    println!("{}", settings4.time == settings.time);
}

static SETTINGS : OnceLock<Settings> = OnceLock::new();

struct Settings {
    name: String,
    flags: u8,
    time: Instant
}

impl Settings {
    pub fn get_setting() -> &'static Settings {
        if let Some(settings) = SETTINGS.get() {
            settings
        } else {
            
            SETTINGS.get_or_init(|| Settings{
                name : "Carl".to_owned(),
                flags: 0,
                time: Instant::now()
            })
        }
    }

    
}
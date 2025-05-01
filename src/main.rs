use rodio::{cpal::{self, traits::HostTrait}, Decoder, DeviceTrait, OutputStream, Sink};
use std::{fs::File, time::UNIX_EPOCH};
use std::io::BufReader;
use std::process::Command;

#[cfg(target_os = "linux")]
const OUTPUT_DEVICE: &str = "bluez_output.F8_5C_7E_59_26_D7.1";

#[cfg(target_os = "windows")]
const OUTPUT_DEVICE: &str = "JBL Clip 4";

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let host = cpal::default_host();
    let devices = host.devices().unwrap();
    let mut output_stream = OutputStream::try_default().expect("could not find default device");

    // THIS IS FOR LINUX
    #[cfg(target_os = "linux")]
    let original_device = switch_device(OUTPUT_DEVICE);

   #[cfg(target_os = "windows")]
    for device in devices {
        if device.name().expect("device has no name").contains(OUTPUT_DEVICE) {
            output_stream = OutputStream::try_from_device(&device.into()).unwrap();
        }
    }

    let stream_handle = output_stream.1;

    let sink = Sink::try_new(&stream_handle).expect("sink could not be created");
    let dur_seconds;
    let sound_name;
    
    if arguments.contains(&"merlinbathroom".to_string()) {
        sink.set_volume(0.5);
        sound_name = format!("light_theme.wav");
        dur_seconds = 20.0;
    } else if arguments.contains(&"meal".to_string()) {
        sink.set_volume(0.7);
        sound_name = format!("CruelAngelsThesis.wav");
        dur_seconds = 23.0;
    } else if arguments.contains(&"wake".to_string()) {
        sink.set_volume(0.6);
        sound_name = format!("HxH_Departure!.wav");
        dur_seconds = 21.0;
    } else if arguments.contains(&"nap".to_string()) {
        sink.set_volume(0.019);
        sound_name = format!("DogSleepMusicShortened.mp3");
        dur_seconds = 6600.0; // 1hr 50min
    } else if arguments.contains(&"sleep".to_string()) {
        sink.set_volume(0.6);
        sound_name = format!("harvest_dawn_oblivion.wav");
        dur_seconds = 20.0;
    } else if arguments.contains(&"stopidle".to_string()) {
        sink.set_volume(0.01);
        sound_name = format!("rumble.wav");
        dur_seconds = 1.0;
    } else if arguments.contains(&"margowake".to_string()) {
        sink.set_volume(0.4);
        sound_name = format!("Peril_Synth.wav");
        dur_seconds = 30.0;
    } else if arguments.contains(&"margobathroom".to_string()) {
        sink.set_volume(0.6);
        sound_name = format!("Red Velvet(Russian Roulette).wav");
        dur_seconds = 15.0;
    } else {
        sound_name = format!("Red Velvet(Russian Roulette).wav");
        sink.set_volume(0.0);
        dur_seconds = 0.0;
    }

    #[cfg(target_os = "linux")]
    let file_path = "/home/dotred/Projects/dog_alarm/sounds/";
    #[cfg(target_os = "windows")]
    let file_path = "./sounds/";

    let file_path = format!("{file_path}{sound_name}");
    

    println!("{file_path}");
    println!("{:?}", std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default());
    let file = BufReader::new(File::open(file_path).expect("sound file not found"));
    let source = Decoder::new(file).unwrap();

    sink.append(source);

    std::thread::sleep(std::time::Duration::from_secs_f32(dur_seconds));
    #[cfg(target_os = "linux")]
    {
        switch_back_device(&original_device);

        std::thread::sleep(std::time::Duration::from_secs_f32(1.0));

        switch_back_device(&original_device);

        std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
    }
    println!("Script Complete");
}

pub fn switch_device(target_device: &str) -> String {
    let og = get_default_sink();
    Command::new("pactl").current_dir("./").arg("set-default-sink").arg(target_device).spawn().expect("Could not switch to target device");

    return og;
}

pub fn switch_back_device(original_device: &str) {
    Command::new("pactl").current_dir("./").arg("set-default-sink").arg(original_device).spawn().expect("could not switch back to og device");
    
    if original_device != get_default_sink() {
        switch_back_device(original_device)
    }
}

pub fn get_default_sink() -> String {
    let output = Command::new("pactl").current_dir("./").arg("get-default-sink").output().expect("could not get default sink");
    let mut default = String::from_utf8(output.stdout).unwrap();
    default.pop(); //removes new line char

    default
}

// pactl set-default-sink bluez_output.F8_5C_7E_59_26_D7.1 
// && 
// paplay --d="bluez_card.F8_5C_7E_59_26_D7" "CruelAngelsThesis.wav"

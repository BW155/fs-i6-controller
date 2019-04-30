use cpal;
use cpal::EventLoop;
use cpal::{Device, StreamData, UnknownTypeInputBuffer};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use vjoy::VJoy;

mod vjoy;
mod vjoy_bindgen;

const HIGH: f32 = 1.0;
const LOW: f32 = -1.0;
const TIME_WINDOW: f32 = 500.0;

fn main() {
    let event_loop = EventLoop::new();
    let devices: Vec<Device> = cpal::devices()
        .filter(|f| f.supported_input_formats().is_ok())
        .filter(|f| f.supported_input_formats().unwrap().next().is_some())
        .collect();
    let device = devices.get(2).unwrap();
    println!("name:{}", device.name());

    let mut supported_formats_range = device
        .supported_input_formats()
        .expect("error while querying formats");
    let format = supported_formats_range
        .next()
        .expect("no supported format?!")
        .with_max_sample_rate();
    let stream_id = event_loop.build_input_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id);

    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        event_loop.run(move |_stream_id, mut stream_data| match stream_data {
            StreamData::Input {
                buffer: UnknownTypeInputBuffer::F32(mut buffer),
            } => {
                let mut v_buff = Vec::new();
                for i in buffer.iter() {
                    let mut number = 0;
                    if number % 8 == 0 {
                        let val = (i * 1.0).round();
                        v_buff.push(val);
                    }
                    number += 1;
                }
                tx.send(v_buff).unwrap();
            }
            _ => (),
        });
    });

    let mut number = 0;
    let mut start_found = false;
    let mut low_count = 0;
    let mut high_count = 0;
    let mut channels: [i64; 6] = [0; 6];
    let mut channel_index = 0;
    let mut started = false;

    let vjoy = VJoy::new();
    println!("{:?}", vjoy);

    loop {
        /*let mut file = OpenOptions::new()
            .append(true)
            .open("testdata.txt")
            .unwrap();
        let mut data = String::new();*/

        let received = rx.recv().unwrap();

        for i in received {
            let val = (i * 1.0).round();

            if high_count > 1600 && i == LOW {
                started = true;
                high_count = 0;
                low_count = 0;
            }

            if started && high_count > 800 {
                started = false;
                channel_index = 0;
            }

            if started && high_count > 0 && i == LOW {
                let mut c_val = ((high_count as f64 - 116.0) / 193.0 * 100.0).round() as i64;
                if c_val > 100 {c_val = 100;}
                if c_val - 1 != channels[channel_index] {
                    channels[channel_index] = c_val;
                }
                channel_index += 1;
            }

            if i == HIGH {
                high_count += 1;
                low_count = 0;
            }
            if i == LOW {
                low_count += 1;
                high_count = 0;
            }
        }
        vjoy.set(channels);
        println!("channels: {:?}", channels);
    }
}

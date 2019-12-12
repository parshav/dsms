extern crate cpal;
use cpal::traits::HostTrait;
use cpal::traits::DeviceTrait;
use cpal::traits::EventLoopTrait;
use cpal::{StreamData, UnknownTypeOutputBuffer};

fn main() {
    println!("Hello, world!");

    let host = cpal::default_host();
    let event_loop = host.event_loop();
    let device = host.default_output_device().expect("No device found");

    let format = device.default_output_format().expect("No format "); // Prints sample rate and format.
    println!("Default! : {:?}", format);

    let stream_id = event_loop.build_output_stream(&device, &format).expect("Error in creating stream id");

    event_loop.play_stream(stream_id).expect("Error in play stream.");
    event_loop.run(move |_s_id, result|{

        let data = match result {
            Ok(d) => d,
            Err(e) => {
                println!("Error in data {:?}", e);
                return;
            }
        };

        // I think matched to what was provided to stream;
        match data {
            StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut _buffer) } => {
                println!("Inside u16");
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut _buffer) } => {
                println!("Inside i16")
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut _buffer) } => {
                println!("Inside f32")
            },
            _ => (),
        };
    });
}

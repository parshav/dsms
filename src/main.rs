extern crate cpal;
use cpal::traits::HostTrait;
use cpal::traits::DeviceTrait;
use cpal::traits::EventLoopTrait;
use cpal::Host;
use cpal::Device;
use cpal::{StreamData, UnknownTypeOutputBuffer};

fn main() {
    println!("Hello, world!");

    let host = cpal::default_host();
    let event_loop = host.event_loop();
    let device = host.default_output_device().expect("No device found");
    println!("Default Device : {:?}", device.name());

    let format = device.default_output_format().expect("No format "); // Prints sample rate and format.

    let device = custom_device(&host);

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
                println!("Inside i32")
            },
            _ => (),
        };
    });
}

fn list_devices(host: &Host) {

    let devices = host.output_devices().expect("Error in multiple output devices");

    for d in devices {
        println!("Name of found device : {:?}", d.name());
    }

}

fn custom_device(host: &Host) -> Device {
    let device = host.output_devices()
        .expect("Error")
        .find( |x| x.name().expect("Error in name").contains( "pulse"))
        .expect("Error on finding the device boi");
    println!("DDD : {:?}", device.name());
    return device;
}

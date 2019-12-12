extern crate cpal;
use cpal::traits::HostTrait;
use cpal::traits::DeviceTrait;

fn main() {
    println!("Hello, world!");

    let host = cpal::default_host();
    let event_loop = host.event_loop();
    let device = host.default_output_device().expect("No device found");

    let format = device.default_output_format().expect("No format found");
    println!("Default! : {:?}", format);
    
}

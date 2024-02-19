use std::time::Duration;

use image::{ImageBuffer, RgbImage};
use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use v4l::video::Capture;

use ffimage_yuv::yuv422::Yuv422;
use ffimage_yuv::yuv::Yuv;

use ffimage::color::{Gray, Rgb};
use ffimage::iter::{BytesExt, ColorConvertExt, PixelsExt};

type AppResult = Result<(),Box<dyn std::error::Error>>;

fn main() -> AppResult {

    // let devices = context::enum_devices();

    // for dev in devices {
    //     println!("{}: {}", dev.index(), dev.name().unwrap());

    // }
    //ELP-USB100W05MT-BL36
    // 3.6mm Lens
    //     v4l2-ctl --device /dev/video0 --set-fmt-video=width=1280,height=720,pixelformat=MJPEG --stream-mmap --stream-to=frame.jpg --stream-count=1
    // https://medium.com/@athul929/capture-an-image-using-v4l2-api-5b6022d79e1d
    // https://gist.github.com/Circuitsoft/1126411/b6e498ddc3b2d5a375684c2ec13a6a0cadf2344e

    let mut dev = Device::new(0).expect("Failed to open device");
    // let controls = dev.query_controls()?;
    // for control in controls {
    //     println!("\t{}", control);
    // }
    let format = dev.format()?;

    // println!("Active format: {}", format);
    // println!("    width: {}",format.width);
    // println!("   height: {}",format.height);

    // let params = dev.params()?;
    // println!("Active parameters:\n{}", params);

    let mut stream = MmapStream::with_buffers(&mut dev, Type::VideoCapture, 4).expect("Failed to create buffer stream");

    let mut counter = 0;
    loop {

        let (buf, meta) = stream.next()?;

        // let len = buf.len();
        // let mut out = [0; 640 * 480 * 2 ];

        // Vec<u8> 
        // Vec<Yuv422<u8, 0, 2, 1, 3>> // after pixels
        // Vec<[Yuv<u8>;2]>  // after colorconvert::<[Yuv<u8>; 2]>()
        // Vec<Yuv<u8>> // after flatten
        // Vec<Rgb<u8>> // after colorconvert::<Rgb<u8>>
        // Vec<[u8;3]> // after bytes;

        let rgb: Vec<u8>   = buf.iter()
            .copied() //.collect();
            .pixels::<Yuv422<u8, 0, 2, 1, 3>>()
            .colorconvert::<[Yuv<u8>; 2]>()
            .flatten()
            .colorconvert::<ffimage::color::Rgb<u8>>()
            .bytes()
            .flatten()
            .collect(); // .write(&mut out);
        
        // // Vec<Gray<u8>>
        // let gray: Vec<u8>  = rgb
        //     .iter()
        //     .copied()
        //     .map(|color| Gray::<u8>::from(color))
        //     .bytes()
        //     .flatten()
        //     .collect();

        println!("len of converted: {}", rgb.len());

        // https://github.com/raymanfx/libv4l-rs/issues/74
        //ImageBuffer<Rgb<u8>, &[u8]> 
        let img: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(
            format.width, 
            format.height, 
            rgb,
        ).unwrap();

        // let img = image::load_from_memory_with_format(&out, image::ImageFormat::Jpeg)?.to_rgb8();

        img.save(format!("test_{}.png",counter));
        counter += 1;

        println!(
            "Buffer size: {}, seq: {}, timestamp: {}",
            buf.len(),
            meta.sequence,
            meta.timestamp
        );
        std::thread::sleep(Duration::from_millis(1000));
    }
    Ok(())
}

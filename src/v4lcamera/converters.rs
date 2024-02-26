use ffimage_yuv::yuv::Yuv;
use ffimage_yuv::yuv420::Yuv420p;
use ffimage_yuv::yuv422::Yuv422;

use ffimage::color::Rgb; // Gray,
use ffimage::iter::{BytesExt, ColorConvertExt, PixelsExt};

pub(crate) fn convert_ir_buffer(buffer: &[u8]) -> Vec<u8> {
	let rgb: Vec<u8> = Yuv420p::pack(&buffer, 640, 512)
		.into_iter()
		.colorconvert::<Rgb<u8>>()
		.bytes()
		.flatten()
		.collect();
	rgb
}

//ELP-USB100W05MT-BL36
// 3.6mm Lens
//     v4l2-ctl --device /dev/video0 --set-fmt-video=width=1280,height=720,pixelformat=MJPEG --stream-mmap --stream-to=frame.jpg --stream-count=1
// https://medium.com/@athul929/capture-an-image-using-v4l2-api-5b6022d79e1d
// https://gist.github.com/Circuitsoft/1126411/b6e498ddc3b2d5a375684c2ec13a6a0cadf2344e
pub(crate) fn convert_rgbvis_buffer(buffer: &[u8]) -> Vec<u8> {
	// Vec<u8>
	// Vec<Yuv422<u8, 0, 2, 1, 3>> // after pixels
	// Vec<[Yuv<u8>;2]>  // after colorconvert::<[Yuv<u8>; 2]>()
	// Vec<Yuv<u8>> // after flatten
	// Vec<Rgb<u8>> // after colorconvert::<Rgb<u8>>
	// Vec<[u8;3]> // after bytes;
	let rgb: Vec<u8> = buffer
		.iter()
		.copied() //.collect();
		.pixels::<Yuv422<u8, 0, 2, 1, 3>>()
		.colorconvert::<[Yuv<u8>; 2]>()
		.flatten()
		.colorconvert::<ffimage::color::Rgb<u8>>()
		.bytes()
		.flatten()
		.collect(); // .write(&mut out);
	rgb
}

/*
pub(crate) fn convert_grayscale_vis_buffer(buffer: &[u8]) -> Vec<u8>{
	todo!("this doesn't really work yet");
	let gray: Vec<u8>  = buffer
		.iter()
		.copied()
		.map(|color| Gray::<u8>::from(color))
		.bytes()
		.flatten()
		.collect();
}
*/

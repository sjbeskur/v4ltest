use v4ltest::*;

fn main() -> v4ltest::AppResult<()> {
	println!("Hello example");
	let camera = v4ltest::camera::OCVCamera::new(0);
	camera.capture_video();
	Ok(())
}

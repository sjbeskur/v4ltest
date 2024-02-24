use opencv::videoio::VideoCapture;

trait ImageSensor {
    fn capture(&self) -> Vec<u8>;
}

pub struct OCVCamera { }

impl OCVCamera{

    pub fn capture(&self) { //-> Result<Vec<u8>, Box<dyn std::error::Error>> {
        //let image = Vec::new();


        //Ok(image);
    }
}
use image;
use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;


// returns the average difference of coordinates in RGB
// may break on particularly large images
pub fn image_diff(im1: &DynamicImage, im2: &DynamicImage) -> Result<f64, String>{
    if im1.width() != im2.width() || im1.height() != im2.height(){
        Err(String::from("image inputs have different dimensions"))
    }else{
        let (w, h) = (im1.width(), im1.height());
        let im1 = im1.to_rgb8();
        let im1data = im1.as_raw();
        let im2 = im2.to_rgb8();
        let im2data = im2.as_raw();
        let mut accum: f64 = 0.0;
        for i in 0..im1data.len(){
            accum += ((im1data[i] as f64) - (im2data[i] as f64)).abs();
        }
        Ok(accum/(im1data.len() as f64))
    }
}

// returns the average difference of coordinates in RGB after norming both images to 0
// may break on particularly large images
pub fn image_diff_normalized(im1: &DynamicImage, im2: &DynamicImage) -> Result<f64, String>{
    if im1.width() != im2.width() || im1.height() != im2.height(){
        Err(String::from("image inputs have different dimensions"))
    }else{
        let (w, h) = (im1.width(), im1.height());
        let im1 = im1.to_rgb8();
        let im1data = im1.as_raw();
        let im1ave = im1data.iter()
            .fold(0.0, |a, b| a+&(*b as f64))
            /(im1data.len() as f64);

        let im2 = im2.to_rgb8();
        let im2data = im2.as_raw();
        let im2ave = im2data.iter()
            .fold(0.0, |a, b| a+&(*b as f64))
            /(im2data.len() as f64);

        let mut accum: f64 = 0.0;
        for i in 0..im1data.len(){
            accum += ((im1data[i] as f64) - (im2data[i] as f64) - (im1ave - im2ave)).abs();
        }
        Ok(accum/(im1data.len() as f64))
    }
}

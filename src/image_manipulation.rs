use image;
use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::GrayImage;
use image::RgbImage;


// returns the average difference of coordinates in RGB
// may break on particularly large images
pub fn image_diff(im1: &DynamicImage, im2: &DynamicImage) -> Result<f64, String>{
    if im1.width() != im2.width() || im1.height() != im2.height(){
        Err(String::from("image inputs have different dimensions"))
    }else{
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
        let im1 = im1.to_rgb8();
        let im1data = im1.as_raw();
        let im1ave = im1data.iter()
            .fold(0.0, |a, b| a+&(*b as f64))
            /(im1data.len() as f64);
        let im1sd = (im1data.iter()
            .fold(0.0, |a, b| {
                let d = im1ave - (*b as f64);
                a + (d*d)
            })/(im1data.len() as f64))
            .sqrt();

        let im2 = im2.to_rgb8();
        let im2data = im2.as_raw();
        let im2ave = im2data.iter()
            .fold(0.0, |a, b| a+&(*b as f64))
            /(im2data.len() as f64);
        let im2sd = (im2data.iter()
            .fold(0.0, |a, b| {
                let d = im2ave - (*b as f64);
                a + (d*d)
            })/(im2data.len() as f64))
            .sqrt();

        let mut accum: f64 = 0.0;
        for i in 0..im1data.len(){
            accum += (((im1data[i] as f64) - im1ave)/im1sd - ((im2data[i] as f64) - im2ave)/im2sd).abs();
        }
        Ok(accum/(im1data.len() as f64))
    }
}

// only compares the middle square
pub fn image_diff_normalized_middle(im1: &DynamicImage, im2: &DynamicImage) -> Result<f64, String>{
    if im1.width() != im2.width() || im1.height() != im2.height(){
        Err(String::from("image inputs have different dimensions"))
    }else{
        let (w, h) = (im1.width(), im1.height());
        image_diff_normalized(
            &im1.crop_imm(w/4, h/4, w/2, h/2),
            &im2.crop_imm(w/4, h/4, w/2, h/2))
    }
}

// returns the average difference of coordinates in RGB after norming both images
// also squares differences
// may break on particularly large images
pub fn image_diff_normalized_squared(im1: &DynamicImage, im2: &DynamicImage) -> Result<f64, String>{
    if im1.width() != im2.width() || im1.height() != im2.height(){
        Err(String::from("image inputs have different dimensions"))
    }else{
        let im1 = im1.to_rgb8();
        let im1data = im1.as_raw();
        let im1ave = im1data.iter()
            .fold(0.0, |a, b| a+&(*b as f64))
            /(im1data.len() as f64);
        let im1sd = (im1data.iter()
            .fold(0.0, |a, b| {
                let d = im1ave - (*b as f64);
                a + (d*d)
            })/(im1data.len() as f64))
            .sqrt();

        let im2 = im2.to_rgb8();
        let im2data = im2.as_raw();
        let im2ave = im2data.iter()
            .fold(0.0, |a, b| a+&(*b as f64))
            /(im2data.len() as f64);
        let im2sd = (im2data.iter()
            .fold(0.0, |a, b| {
                let d = im2ave - (*b as f64);
                a + (d*d)
            })/(im2data.len() as f64))
            .sqrt();

        let mut accum: f64 = 0.0;
        for i in 0..im1data.len(){
            let diff = (((im1data[i] as f64) - im1ave)/im1sd - ((im2data[i] as f64) - im2ave)/im2sd).abs();
            accum += diff*diff;
        }
        Ok(accum/(im1data.len() as f64))
    }
}


// only compares whether a pixel is greater than average or less than average
// may break on particularly large images
pub fn image_diff_highlights(im1: &DynamicImage, im2: &DynamicImage) -> Result<f64, String>{
    if im1.width() != im2.width() || im1.height() != im2.height(){
        Err(String::from("image inputs have different dimensions"))
    }else{
        let im1 = im1.to_luma8();
        let im1data = im1.as_raw();
        let im1ave = im1data.iter()
            .fold(0.0, |a, b| a+&(*b as f64))
            /(im1data.len() as f64);

        let im2 = im2.to_luma8();
        let im2data = im2.as_raw();
        let im2ave = im2data.iter()
            .fold(0.0, |a, b| a+&(*b as f64))
            /(im2data.len() as f64);

        let mut accum: f64 = 0.0;
        for i in 0..im1data.len(){
            if ((im1data[i] as f64) > im1ave) != ((im2data[i] as f64) > im2ave){
                accum += 1.0;
            }
        }
        Ok(accum/(im1data.len() as f64))
    }
}

pub fn sharpen(im: &GrayImage) -> GrayImage{
    let (w, h) = (im.width(), im.height());
    let mut imout = GrayImage::new(w, h);

    for x in 0..w{
        for y in 0..h{
            let mut value: i32 = 0;
            let mut neighbors = 0;
            for dx in -1..2{
                for dy in -1..2{
                    let x2 = (x as i32) + dx;
                    let y2 = (y as i32) + dy;

                    if dx == 0 && dy == 0{
                        continue;
                    }else if x2 >= 0 && x2 < (w as i32) && y2 >= 0 && y2 < (h as i32){
                        value -= (im.get_pixel(x2 as u32, y2 as u32).0)[0] as i32;
                        neighbors += 1
                    }
                }
            }
            value += ((im.get_pixel(x, y).0)[0] as i32) * (neighbors*2);
            value = value/neighbors;

            imout.put_pixel(x, y, image::Luma([value as u8]));
        }
    }

    imout
}

pub fn max_contrast(im: &DynamicImage) -> DynamicImage{
    let (w, h) = (im.width(), im.height());
    let mut imout = RgbImage::new(w, h);
    let im = im.to_rgb8();
    let mut imave = [0.0; 3];

    for x in 0..w{
        for y in 0..h{
            let px = im.get_pixel(x, y).0;
            for i in 0..3{
                imave[i] += px[i] as f64;
            }
        }
    }

    for i in 0..3{
        imave[i] = imave[i]/((w*h) as f64);
    }

    let mut imsd = [0.0; 3];
    for x in 0..w{
        for y in 0..h{
            let px = im.get_pixel(x, y).0;
            let diff = [
                imave[0] - (px[0] as f64),
                imave[1] - (px[1] as f64),
                imave[2] - (px[2] as f64),
            ];
            for i in 0..3{
                imsd[i] += diff[i]*diff[i];
            }
        }
    }

    for i in 0..3{
        imsd[i] = (imsd[i]/((w*h) as f64)).sqrt();
    }

    // I'm just eyeballing this number
    let cutoff = [
        imave[0] + imsd[0]*0.5,
        imave[1] + imsd[1]*0.5,
        imave[2] + imsd[2]*0.5,
    ];

    for x in 0..w{
        for y in 0..h{
            let mut color = [0, 0, 0];
            let px = im.get_pixel(x, y).0;
            for i in 0..3{
                if px[i] as f64 > cutoff[i]{
                    color[i] = 255;
                }else{
                    color[i] = 0;
                }
            }

            imout.put_pixel(x, y, image::Rgb(color));
        }
    }

    DynamicImage::ImageRgb8(imout)
}

pub fn max_contrast_grayscale(im: &DynamicImage) -> DynamicImage{
    let (w, h) = (im.width(), im.height());
    let mut imout = GrayImage::new(w, h);
    let im = im.to_luma8();
    let imdata = im.as_raw();
    let imave = imdata.iter()
        .fold(0.0, |a, b| a+&(*b as f64))
        /(imdata.len() as f64);
    let imsd = (imdata.iter()
        .fold(0.0, |a, b| {
            let d = imave - (*b as f64);
            a + (d*d)
        })/(imdata.len() as f64))
        .sqrt();

    // I'm just eyeballing this number
    let cutoff = imave + imsd*0.5;

    for x in 0..w{
        for y in 0..h{
            if(im.get_pixel(x, y).0)[0] as f64 > cutoff{
                imout.put_pixel(x, y, image::Luma([255]));
            }else{
                imout.put_pixel(x, y, image::Luma([0]));
            }
        }
    }
    DynamicImage::ImageLuma8(imout)
}

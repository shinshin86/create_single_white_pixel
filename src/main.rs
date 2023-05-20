extern crate image;

use image::{Rgba, ImageBuffer};

fn create_white_pixel_image(file_path: &str) -> image::ImageResult<()> {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(1, 1, |_x, _y| {
        Rgba([255, 255, 255, 0])
    });

    img.save(file_path)
}

fn main() {
    let file_path = "1x1.jpg";
    match create_white_pixel_image(file_path) {
        Ok(_) => println!("Image successfully created."),
        Err(e) => eprintln!("Error creating image: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_create_white_pixel_image() {
        let file_path = "1x1_test.jpg";
        assert!(create_white_pixel_image(file_path).is_ok());
        assert!(Path::new(file_path).exists());
        std::fs::remove_file(file_path).unwrap();
    }
}

use fast_image_resize as fr;
use image::codecs::jpeg::JpegEncoder;
use image::{ColorType, ImageEncoder};
use std::io::BufWriter;
use std::{num::NonZeroU32, path::PathBuf};

use image::io::Reader as ImageReader;

pub(crate) fn resize(from: PathBuf, to: PathBuf) -> Result<(), ()> {
    let img = ImageReader::open(from.as_os_str().to_str().expect("File must Exist!"))
        .unwrap()
        .decode()
        .unwrap();

    let width = NonZeroU32::new(img.width()).unwrap();
    let height = NonZeroU32::new(img.height()).unwrap();

    let src_image = fr::Image::from_vec_u8(
        width,
        height,
        img.to_rgba8().into_raw(),
        fr::PixelType::U16x4,
    )
    .unwrap();

    let dst_width = NonZeroU32::new(width.get()).unwrap();
    let dst_height = NonZeroU32::new(height.get()).unwrap();
    let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

    let mut dst_view = dst_image.view_mut();

    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();

    let mut result_buf = BufWriter::new(Vec::new());
    JpegEncoder::new(&mut result_buf)
        .write_image(
            dst_image.buffer(),
            dst_width.get(),
            dst_height.get(),
            ColorType::Rgba32F,
        )
        .unwrap();

    Ok(())
}

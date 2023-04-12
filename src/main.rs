use image::{ImageBuffer, Rgb, RgbImage};

fn main() {
    let width = 250;
    let height = 250;
    let pixel_size = 5;
    let base_line = height / 2;
    let step = 0.01;
    let zoom = 1;

    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::new(width, height);

    // Full white
    image = graph(
        blank(image, base_line),
        pixel_size,
        step,
        zoom as f64,
        base_line,
    );
    // image = blank(image);
    save_image(image, 14);
}

fn math_fn(x: f64) -> f64 {
    let w: f64 = 0.5;
    let b: f64 = 10.;
    return w * x + b;

    // return x.powf(3.);

    // return x.sin();

    // return x.log10();
    
    // if x != 0. {
    //     return 1. / x;
    // } else {
    //     return 1.;
    // }

    // if x != 0. {
    //     return (1. + 1. / x).powf(x);
    // } else {
    //     return -1.;
    // }

    // return -1_f64.powf(x);

    // return x.abs();
}

fn graph(
    mut img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    _pixel_size: u32,
    step: f64,
    zoom: f64,
    base_line: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let height = img.height();
    let width = img.width();
    let color: [u8; 3] = [255, 0, 0];

    let mut x: f64 = (width / 2) as f64 * -1.;

    while x < (width / 2).into() {
        let y = math_fn(x);

        println!("x: {} | y: {}", x, y);

        let (new_x, new_y) = calc_point(x, y as f64, height as f64, width as f64, zoom, base_line);

        println!("BEFORE: new_x: {} | new_y: {}", new_x, new_y);

        println!("AFTER: new_x: {} | new_y: {}", new_x, new_y);

        if new_x < 0 || new_y < 0 {
            x += step;
            continue;
        } else if new_x < width as i32 && new_y < height as i32 {
            img.put_pixel(new_x as u32, new_y as u32, Rgb(color));
        } else if new_x > width as i32 || new_y > height as i32 {
            x += step;
            continue;
        }
        x += step;
    }

    img
}

fn blank(
    mut img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    base_line: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let color: [u8; 3] = [255, 255, 255];
    let color_secondary: [u8; 3] = [10, 10, 10];
    let line_height = 10;
    for x in 0..img.width() {
        for y in 0..img.height() {
            img.put_pixel(x, y, Rgb(color));
        }
    }
    for i in 1..img.width() {
        let x = i as u32;
        println!("x: {}", x);
        if x >= img.width() {
            break;
        } else if x % 50 == 0 {
            for i in 0..line_height {
                img.put_pixel(x, img.height() - base_line + line_height/2 - i, Rgb(color_secondary));
            }
        } else if x % 5 == 0 {
            img.put_pixel(x, img.height() - base_line, Rgb(color_secondary));
        }
    }
    for j in 1..img.height() {
        let y = j as u32;
        println!("y: {}", y);
        if y >= img.height() {
            break;
        } else if y % 50 == 0 {
            for i in 0..line_height {
                img.put_pixel(img.width() / 2 + line_height/2 - i, y, Rgb(color_secondary));
            }
        } else if y % 5 == 0 {
            img.put_pixel(img.width() / 2, y, Rgb(color_secondary));
        }
    }
    img
}

fn save_image(img: ImageBuffer<Rgb<u8>, Vec<u8>>, img_num: u32) {
    let outputpath: String = "./output/".to_string() + &img_num.to_string() + &".png".to_string();
    img.save(outputpath).expect("Writing image to png");
}

fn calc_point(x: f64, y: f64, h: f64, w: f64, zoom: f64, base_line: u32) -> (i32, i32) {
    return (
        (x * zoom + w / 2.).round() as i32,
        (h - y * zoom - h + base_line as f64).round() as i32,
    );
}

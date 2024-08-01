use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn pixel_filter(mut buffer: Vec<u8>,canvas_width :u32,canvas_height :u32,dot_size :u32) -> Vec<u8> {
    let width = canvas_width as usize;
    let height = canvas_height as usize;
    let dot_size = dot_size as usize;

    // for y in (0..height).step_by(dot_size) {
    //     for x in (0..width).step_by(dot_size) {
    //         let mut r = 0;
    //         let mut g = 0;
    //         let mut b = 0;
    //
    //         for dy in 0..dot_size {
    //             for dx in 0..dot_size {
    //                 let i = ((y + dy) * width + (x + dx)) * 4; // RGBAなので4倍
    //                 if i + 3 < buffer.len() {
    //                     r += buffer[i] as u32;
    //                     g += buffer[i + 1] as u32;
    //                     b += buffer[i + 2] as u32;
    //                 }
    //             }
    //         }
    //
    //         // ドット内のすべてのピクセルに平均色を設定
    //         r /= (dot_size * dot_size) as u32;
    //         g /= (dot_size * dot_size) as u32;
    //         b /= (dot_size * dot_size) as u32;
    //
    //         for dy in 0..dot_size {
    //             for dx in 0..dot_size {
    //                 let i = ((y + dy) * width + (x + dx)) * 4;
    //                 if i + 3 < buffer.len() {
    //                     buffer[i] = r as u8;
    //                     buffer[i + 1] = g as u8;
    //                     buffer[i + 2] = b as u8;
    //                     buffer[i + 3] = 255; // アルファ値
    //                 }
    //             }
    //         }
    //     }
    // }


    // グレースケールに変換
    for i in (0..buffer.len()).step_by(4) {
        let avg = (buffer[i] as u16 + buffer[i + 1] as u16 + buffer[i + 2] as u16) / 3;
        buffer[i] = avg as u8;
        buffer[i + 1] = avg as u8;
        buffer[i + 2] = avg as u8;
    }

    // Sobel フィルタを使ってエッジ検出
    let mut edges = vec![0u8; buffer.len() / 4];
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let gx = -1 * buffer[((y - 1) * width + (x - 1)) * 4] as i16
                + buffer[((y - 1) * width + (x + 1)) * 4] as i16
                - 2 * buffer[((y) * width + (x - 1)) * 4] as i16
                + 2 * buffer[((y) * width + (x + 1)) * 4] as i16
                - buffer[((y + 1) * width + (x - 1)) * 4] as i16
                + buffer[((y + 1) * width + (x + 1)) * 4] as i16;

            let gy = buffer[((y - 1) * width + (x - 1)) * 4] as i16
                + 2 * buffer[((y - 1) * width + (x)) * 4] as i16
                + buffer[((y - 1) * width + (x + 1)) * 4] as i16
                - buffer[((y + 1) * width + (x - 1)) * 4] as i16
                - 2 * buffer[((y + 1) * width + (x)) * 4] as i16
                - buffer[((y + 1) * width + (x + 1)) * 4] as i16;

            let mag = int_sqrt((gx * gx + gy * gy) as u32).min(255) as u8;
            edges[y * width + x] = mag;
        }
    }

    // エッジを黒、それ以外を白に設定
    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) * 4;
            if edges[y * width + x] > 50 {
                // エッジ
                buffer[index] = 0;
                buffer[index + 1] = 0;
                buffer[index + 2] = 0;
                buffer[index + 3] = 255;
            } else {
                // エッジ以外
                buffer[index] = 255;
                buffer[index + 1] = 255;
                buffer[index + 2] = 255;
                buffer[index + 3] = 255;
            }
        }
    }

    buffer
}

fn int_sqrt(n: u32) -> u32 {
    let mut x = n;
    let mut y = (x + 1) >> 1;
    while y < x {
        x = y;
        y = (x + n / x) >> 1;
    }
    x
}
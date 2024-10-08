use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct DataPackage {
    bytes: Vec<u8>,
    chks: Vec<u8>,
    text:String,
}

#[wasm_bindgen]
impl DataPackage {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: Vec<u8>, chks: Vec<u8>,text:String) -> Self {
        DataPackage { bytes, chks,text }
    }

    #[wasm_bindgen(getter)]
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn chks(&self) -> Vec<u8> {
        self.chks.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }
}
#[wasm_bindgen]
pub fn pixel_filter(mut buffer: Vec<u8>,canvas_width :u32,canvas_height :u32) -> DataPackage {
    let width = canvas_width as usize;
    let height = canvas_height as usize;
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

    let mut text = String::new();
    let mut count = 0;
    let mut arr_chk = vec![0u8; width * height];
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
                arr_chk[count] = 1;
                text.push_str("@");
            } else {
                // エッジ以外
                buffer[index] = 255;
                buffer[index + 1] = 255;
                buffer[index + 2] = 255;
                buffer[index + 3] = 255;
                arr_chk[count] = 0;
                text.push_str("_");
            }
            count += 1;
        }
        text.push_str("\r\n");
    }

    DataPackage::new(Vec::from(buffer.as_slice()), arr_chk,text)
}


#[wasm_bindgen]
pub fn edge_black_filter(mut buffer: Vec<u8>,canvas_width :u32,canvas_height :u32) -> Vec<u8> {
    let width = canvas_width as usize;
    let height = canvas_height as usize;

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
                // buffer[index + 3] = 255;
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

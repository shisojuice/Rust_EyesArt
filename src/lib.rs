use std::io::{Cursor};
use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageFormat, Rgba};
use base64::{engine::general_purpose, Engine as _};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn trick_png(arrow: u32) -> String{
    let mut img = DynamicImage::new_rgba8(160,160);
    draw_filled_rect_mut(&mut img,  Rect::at(0, 0).of_size(160, 160), Rgba([255, 228, 196, 255]));
    // 目の描画
    let eye_x = img.width() / 2;
    let eye_y = img.height() / 2;
    draw_filled_circle_mut(&mut img, (eye_x as i32, eye_y as i32), 45, Rgba([255, 255, 255, 255]));

    if arrow == 0
    {
        draw_filled_circle_mut(&mut img, (eye_x as i32, eye_y as i32), 15, Rgba([0, 0, 0, 255]));
    }
    // 左
    if arrow == 1
    {
        draw_filled_circle_mut(&mut img, (eye_x as i32 - 15, eye_y as i32), 15, Rgba([0, 0, 0, 255]));
    }
    // 下
    if arrow == 2
    {
        draw_filled_circle_mut(&mut img, (eye_x as i32, eye_y as i32 + 15), 15, Rgba([0, 0, 0, 255]));
    }
    // 上
    if arrow == 3
    {
        draw_filled_circle_mut(&mut img, (eye_x as i32, eye_y as i32 - 15), 15, Rgba([0, 0, 0, 255]));
    }
    // 右
    if arrow == 4
    {
        draw_filled_circle_mut(&mut img, (eye_x as i32 + 15, eye_y as i32), 15, Rgba([0, 0, 0, 255]));
    }

    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png).unwrap();
    let base64_string = general_purpose::STANDARD.encode(buffer.get_ref());
    // データURL形式で返す
    format!("data:image/png;base64,{}", base64_string)
}

use image;

fn main() {
    // 白黒をRgbで定義
    let white = image::Rgb::<u8>([255, 255, 255]);
    // 赤色をRgbで定義
    let red = image::Rgb::<u8>([255, 90, 90]);
    // 1マスの定義
    let w = 64;
    let draw = |x, y| {
        let (xi, yi) = (x / w, y / w);
        match (xi % 2, yi % 2) {
            (0, 0) => white,
            (1, 0) => red,
            (0, 1) => red,
            (1, 1) => white,
            (_, _) => panic!("error"),
        }
    };
    // クロージャーを指定してImageBufferを生成
    let img = image::ImageBuffer::from_fn(512, 512, draw);
    // ファイルへ保存
    img.save("ichimatsu.png").unwrap();
}

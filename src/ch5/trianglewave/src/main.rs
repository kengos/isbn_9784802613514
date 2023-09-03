use hound;
const SAMPLE_RATE: f32 = 44_100.0;

fn main() {
    // WAVファイルのフォーマットを指定
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    // WavWriterのオブジェクトを生成
    let mut fw = hound::WavWriter::create("tri.wav", spec).unwrap();
    // 短形波を生成
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    // メロディを生成
    [60, 64, 67, 64, 60, 64, 67, 72].iter().for_each(|no| {
        wav.extend(tri_wave(*no, calc_len(bpm, 8), 0.5));
    });
    // ファイルに書き込む
    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}

fn noteno_to_hz(no: i32) -> f32 {
    440.0 * 2.0f32.powf((no - 69) as f32 / 12.0)
}

// n音符のサンプル数を計算
fn calc_len(bpm: usize, n: usize) -> usize {
    let base_len = (60.0 / bpm as f32) * SAMPLE_RATE;
    ((4.0 / n as f32) * base_len) as usize
}

// 三角波を生成する
fn tri_wave(noteno: i32, len: usize, gain: f32) -> Vec<f32> {
    let tone = noteno_to_hz(noteno); // 周波数を得る
    let form_samples = SAMPLE_RATE / tone; // 周期を得る
    let mut wav: Vec<f32> = vec![0.0; len];
    let half_fs = form_samples / 2.0; // 周期の半分
    for i in 0..len {
        let hi = i as f32 / half_fs;
        let v: f32 = 2.0 * (hi % 1.0) - 1.0;
        let is_climbing = hi.floor() as usize % 2 == 0;
        wav[i] = if is_climbing { v } else { -v };
    }
    // 音量調節
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}

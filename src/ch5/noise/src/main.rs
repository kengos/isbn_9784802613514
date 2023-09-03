use hound;
use rand::prelude::*;
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
    let mut fw = hound::WavWriter::create("noise.wav", spec).unwrap();
    // 短形波を生成
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    // フルレンジのノイズを生成
    wav.extend(noise(2.0, -1.0, calc_len(bpm, 2)));
    wav.extend(noise(0.2, 0.8, calc_len(bpm, 2)));
    wav.extend(noise(0.8, -1.0, calc_len(bpm, 2)));
    // ファイルに書き込む
    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}

// n音符のサンプル数を計算
fn calc_len(bpm: usize, n: usize) -> usize {
    ((4.0 / n as f32) * (60.0 / bpm as f32) * SAMPLE_RATE) as usize
}

// ノイズを生成する
fn noise(range: f32, shift: f32, len: usize) -> Vec<f32> {
    let mut wav: Vec<f32> = vec![0.0; len];
    let mut rng = rand::thread_rng();
    for i in 0..len {
        wav[i] = rng.gen::<f32>() * range + shift;
    }
    // 音量調整
    wav.into_iter().map(|v| (v * 0.5) as f32).collect()
}

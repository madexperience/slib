use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub fn play_audio_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = file_path.to_string();
    std::thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default()
            .expect("출력 스트림 생성 실패");
        let sink = Sink::try_new(&stream_handle)
            .expect("Sink 생성 실패");

        let file = File::open(&file_path)
            .expect("파일 열기 실패");
        let source = Decoder::new(BufReader::new(file))
            .expect("디코더 생성 실패");

        sink.append(source);
        sink.sleep_until_end();
    });
    Ok(())
}

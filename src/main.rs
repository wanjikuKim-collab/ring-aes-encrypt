mod mp4_reader;
fn main() {
    let _reader = mp4_reader::MP4Reader::read_file("How to use FFMPEG.mp4").expect("File not found");
}

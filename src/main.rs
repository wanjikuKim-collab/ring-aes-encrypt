mod mp4_reader;
mod encryption;

fn main() {
    let reader_a = mp4_reader::MP4Reader::read_file("/home/wanjiku/Development/code/rust/ring-aes-encrypt/src/How to use FFMPEG.mp4");

    match reader_a {
        Ok(reader) => {
            // Access and print some bytes from the data
            let data_slice = &reader.data[..10];
            println!("Read some bytes from file: {:?}", reader.data);
        },
        Err(err) => {
            println!("Error reading file: {:?}", err);
            // Handle the error appropriately (e.g., exit program, retry)
        },
    }
}

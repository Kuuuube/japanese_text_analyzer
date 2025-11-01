use std::{
    fs::File,
    io::{Read, Seek},
    path::PathBuf,
};

/// Safely reads a UTF8 file buffered without chopping multi-byte characters in half.
///
/// Iterator returns `None` upon finding invalid UTF8 (enable `read_unsafe` to override this) and once the file has been completed.
///
/// Iterator `String` chunks will always be less than or equal to the provided `buffer_size`.
pub struct Utf8BufReader {
    file: File,
    buffer: Vec<u8>,
    end_of_file: bool,
    read_unsafe: bool,
}

impl Utf8BufReader {
    pub fn new(file_path: &PathBuf, buffer_size: usize) -> Result<Self, std::io::Error> {
        let file = File::open(file_path)?;
        Ok(Self {
            file: file,
            buffer: vec![0u8; buffer_size],
            end_of_file: false,
            read_unsafe: false,
        })
    }

    #[allow(dead_code)]
    pub fn read_unsafe(&mut self, read_unsafe: bool) {
        self.read_unsafe = read_unsafe;
    }
}

impl Iterator for Utf8BufReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_of_file {
            return None;
        }

        let buffer_length = self.buffer.len();
        let bytes_filled = self.file.read(&mut self.buffer).ok()?;
        self.end_of_file = bytes_filled < buffer_length;
        self.buffer.resize(bytes_filled, b'\0');
        let (file_contents, seek_position) = parse_utf8_buffer(&self.buffer, self.read_unsafe);
        if file_contents.len() == 0 {
            return None;
        }
        let _ = self.file.seek(std::io::SeekFrom::Current(
            seek_position as i64 - buffer_length as i64,
        ));
        return Some(file_contents);
    }
}

fn parse_utf8_buffer(file_buffer: &Vec<u8>, read_unsafe: bool) -> (String, usize) {
    let mut seek_position: usize = file_buffer.len();
    let utf8_string = match std::str::from_utf8(file_buffer) {
        Ok(ok) => ok.to_string(),
        Err(err) => {
            seek_position = err.valid_up_to();
            if seek_position == 0 && read_unsafe {
                seek_position = file_buffer.len();
                unsafe { std::str::from_utf8_unchecked(&file_buffer) }.to_string()
            } else {
                //should never panic as long as seek_position = err.valid_up_to()
                std::str::from_utf8(&file_buffer[0..seek_position])
                    .unwrap()
                    .to_string()
            }
        }
    };
    (utf8_string, seek_position)
}

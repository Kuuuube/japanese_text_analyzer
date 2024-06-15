use sudachi::dic::{dictionary::JapaneseDictionary, storage::{Storage, SudachiDicData}};

pub fn make_sudachi_dict() -> Result<JapaneseDictionary, Box<dyn std::error::Error>> {
    let embedded_dictionary = decode_zstd(include_bytes!("./system_full.dic.zst"))?;
    let dictionary_file_data = SudachiDicData::new(Storage::Owned(embedded_dictionary));
    let config = sudachi::config::Config::new_embedded()?;
    let dictionary = JapaneseDictionary::from_embedded_storage(&config, dictionary_file_data)?;
    return Ok(dictionary);
}

fn decode_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let bound = zstd_safe::decompress_bound(&data).expect("zstd_safe::decompress_bound failed");
    let mut decompressed: Vec<u8> = Vec::with_capacity(bound.try_into()?);
    zstd_safe::decompress(&mut decompressed, &data).expect("zstd_safe::decompress failed");
    return Ok(decompressed);
}

use sudachi::dic::{dictionary::JapaneseDictionary, storage::{Storage, SudachiDicData}};

pub fn make_sudachi_dict() -> Result<JapaneseDictionary, Box<dyn std::error::Error>> {
    let embedded_dictionary = decode_zstd(include_bytes!("./system_full.dic.zst"))?;
    let dictionary_file_data = SudachiDicData::new(Storage::Owned(embedded_dictionary));
    let config = sudachi::config::Config::new_embedded()?;
    let dictionary = JapaneseDictionary::from_cfg_storage_with_embedded_chardef(&config, dictionary_file_data)?;
    return Ok(dictionary);
}

pub fn get_mode() -> sudachi::analysis::Mode {
    /* Mode reference for sudachi system_full.dic
        A：医薬/品/安全/管理/責任/者
        B：医薬品/安全/管理/責任者
        C：医薬品安全管理責任者

        A：消費/者/安全/調査/委員/会
        B：消費者/安全/調査/委員会
        C：消費者安全調査委員会

        A：さっぽろ/テレビ/塔
        B：さっぽろ/テレビ塔
        C：さっぽろテレビ塔

        A：カンヌ/国際/映画/祭
        B：カンヌ/国際/映画祭
        C：カンヌ国際映画祭

        When testing on real text this doesn't apply as extremely but it should be kept in mind.
        A：Perfect at creating words that aren't too compounded to be found in definition dictionaries. But splits almost all compound words.
        B：Nearly perfect at creating words definition dictionaries will contain. In rare cases it may create a compound word that is hard to find.
        C：In some cases can create ridiculously long compound words that no definition dictionaries will contain.
     */
    return sudachi::analysis::Mode::B;
}

fn decode_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let bound = zstd_safe::decompress_bound(&data).expect("zstd_safe::decompress_bound failed");
    let mut decompressed: Vec<u8> = Vec::with_capacity(bound.try_into()?);
    zstd_safe::decompress(&mut decompressed, &data).expect("zstd_safe::decompress failed");
    return Ok(decompressed);
}

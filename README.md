# Japanese Text Analyzer

Analysis tool for ocr files in Mokuro processed manga.

## Usage

```
japanese_text_analyzer directory_path
```

## Building

1. Download `sudachi-dictionary-latest-full.zip` from [SudachiDict](http://sudachi.s3-website-ap-northeast-1.amazonaws.com/sudachidict/).

2. Extract `system_full.dic`.

3. Compress `system_full.dic` using zstd with the filename `system_full.dic.zst`

4. Place the file in `./src/system_full.dic.zst`

```
cargo build --release
```

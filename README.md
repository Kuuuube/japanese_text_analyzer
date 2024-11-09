# Japanese Text Analyzer

Analysis tool for ocr files in Mokuro processed manga.

## Usage

```
japanese_text_analyzer mokuro_manga_directory_path OPTIONS
```

### Options

`--mokurojson` (Default): Searches only for `.json` files in the specified path.

    Note: The Mokuro `_ocr` json files **must** be present.

`--txt`: Searches only for `.txt` files in the specified path.

`--any`: Searches for all files in the specified path.

## Sample Output

`analysis.txt`
```
./sample_manga/
----------------------------------------------------------------------------
Number of Japanese characters: 43811
Number of kanji characters: 10952
Number of unique kanji: 1082
Number of unique kanji appearing only once: 285 (26.34% of unique kanji)
Number of words in total: 25204
Number of unique words: 3519 (13.96% of all words)
Number of words appearing only once: 2018 (57.35% of unique words)
Average volume length in characters: 14603 (3 total volumes)
Average page length in characters: 103 (422 total pages)
Average textbox length in characters: 11 (shortest: 1) (longest: 254) (4302 total textboxes)
```

`word_list.csv`
```
て	831
の	805
に	710
た	702
です	555
は	528
で	521
が	508
ん	504
... (3510 more lines)
```

## Building

Linux:
```
./setup.sh
cargo build --release
```

Windows:
```
setup.bat
cargo build --release
```

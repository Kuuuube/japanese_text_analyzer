# Japanese Text Analyzer

Analysis tool for ocr files in Mokuro processed manga. Also supports miscellaneous files.

## Usage

```
japanese_text_analyzer directory_or_file_path OPTIONS
```

## Options

- `-h` `--help`: Print a help message and exit.

- `--mokurojson` (Default): Searches only for `.json` files in the specified path.

    Note: The Mokuro `_ocr` json files **must** be present.

- `--mokuro`: Searches only for `.mokuro` files in the specified path.

    Note: The Mokuro `.mokuro` files **must** be present.

- `--any`: Searches for all files in the specified path.

- `--any=EXTENSION`: Searches for all files matching the file extension in the specified path.

## Examples

```
japanese_text_analyzer ./mokuro_manga_path/
```
```
japanese_text_analyzer "./example path/" --any
```
```
japanese_text_analyzer "./example path/" --any=.html
```

## Sample Output

`analysis.txt` (Stats on the analyzed text)
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

`word_list.csv` (Deduped list of words along with the number of times they were found in the analyzed text)
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

`word_list_raw.csv` (Unsorted list of words found in the analyzed text)
```
まぁ
まぁ
話し
て
き
まし
... (25198 more lines)
```

`kanji_list_csv` (Deduped list of kanji along with the number of times they were found in the analyzed text)
```
前	320
川	230
私	208
水	187
清	186
... (1077 more lines)
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

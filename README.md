# blconv

*This project is in its beginning stages as of 7/28/24 and is only functional for basic image conversion.*

An easy-to-use CLI media format converter, written in Rust!

## (Intended) Usage

```bash
blconv <filename.ext> <target_ext>
# E.g.
blconv example.png jpg 
# Output: 'example.png' successfully converted to 'example.jpg'!
```
Be careful though:
```bash
blconv example.jpg tiff
# Output: 
# thread 'main' panicked at src/main.rs:7:74:
# called `Result::unwrap()` on an `Err` value: UnsupportedFormatError { format: "tiff" }
```
While a panic isn't the ideal behavior, it does get the message across for now. And it even shows what the problem format was pretty clearly!

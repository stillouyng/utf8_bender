# utf8_bender 🤪

> "There is no practical use - it's art! 🎨"

`utf8_bender` implements the revolutionary `BogusEncoding` standard:
- 100% slower than `as_bytes()`!
- 200% more spaces in output!
- 300% more existential questions per line of code!

## Usage
```rust
use utf8_bender::encode;

fn main() {
    let result = encode("😊").unwrap();
    println!("Your smile in binary: {}", result);  // 11110000 10011111 10011000 10001010 
}
```
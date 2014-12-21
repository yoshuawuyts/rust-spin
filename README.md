# rust-spin
[![build status][travis-image]][travis-url]

Little terminal spinner lib. Inspired by [go-spin][go-spin].

## Installation
```bash
$ git clone https://github.com/yoshuawuyts/rust-spin
```

## Usage
```rust
extern crate spin

use std::time::Duration;
use std::io::timer;
use spin;

fn main() {
  let val = spin('default');

  for i in range(0, 30) {
    print!("\r  \033[36mcomputing\033[m {} ", val(i));
    timer::sleep(Duration::milliseconds(200));
  }
}
```

## See Also
- [go-spin][go-spin]

## License
[MIT](https://tldrlegal.com/license/mit-license)

[travis-image]: https://img.shields.io/travis/yoshuawuyts/rust-spin.svg?style=flat-square
[travis-url]: https://travis-ci.org/yoshuawuyts/rust-spin

[go-spin]: https://github.com/tj/go-spin

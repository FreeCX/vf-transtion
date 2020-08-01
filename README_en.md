transition
---
Automatic video creation from two photos by transition between them according to the selected effect

Implemented in two languages:
- [Rust](./src/main.rs)
- [Python¹](./src/transition.py)

Common dependencies:
- ffmpeg

Python dependencies:
- argparse
- numpy
- PIL

Rust dependencies:
- argparse
- lodepng
- rgb

Implemented effects:
- [2.5D effect, i dont know how it this called](https://gfycat.com/coldfavorableflicker)
- [vertical transition](https://gfycat.com/silentimaginativegodwit)

Files for the test [can be found here](./demo/)

¹ only the first effect is implemented

transition
---
Автоматическое создание [видео из двух фото с плавным переходом между ними](https://gfycat.com/coldfavorableflicker) (2.5D эффект, хз как оно правильно называется).

Реализовано на двух языках:
- [Код на Rust](./src/main.rs)
- [Код на Python](./src/transition.py)

Общие зависимости:
- ffmpeg

Зависимости python:
- argparse
- numpy
- PIL

Зависимости rust:
- lodepng
- rgb

Файлы для теста [можно найти здесь](./demo/)
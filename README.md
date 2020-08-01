transition
---
Автоматическое создание видео из двух фото с плавным переходом между ними

Реализовано на двух языках:
- [Код на Rust](./src/main.rs)
- [Код на Python¹](./src/transition.py)

Общие зависимости:
- ffmpeg

Зависимости python:
- argparse
- numpy
- PIL

Зависимости rust:
- argparse
- lodepng
- rgb

Реализованые эффекты:
- [2.5D эффект, хз как оно правильно называется](https://gfycat.com/coldfavorableflicker)
- [вертикальный переход](https://gfycat.com/silentimaginativegodwit)

Файлы для теста [можно найти здесь](./demo/)

¹ реализован только первый эффект
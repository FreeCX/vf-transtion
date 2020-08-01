transition
---
[english version](README_en.md)

Автоматическое создание видео из двух фото переходом между ними по выбранному эффекту

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

Реализованные эффекты:
- [2.5D эффект, хз как оно правильно называется](https://gfycat.com/coldfavorableflicker)
- [вертикальный переход](https://gfycat.com/silentimaginativegodwit)

Почитать про этапы реализации можно по [ссылке](https://freecx.github.io/blog/2020/07/23/2.5d-effect)

Файлы для теста [можно найти здесь](./demo/)

¹ реализован только первый эффект

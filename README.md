# trosna-file-demo

Минимальные примеры создания файла временного ряда с расширением **`.trosna`**
— на Rust, на Python и из командной строки. Во всех трёх примерах сохраняется
один и тот же захардкоженный ряд из пяти измерений комнатного термометра.

Библиотека и спецификация формата: **<https://github.com/89605502155/trosna-file>**
(подробное руководство — [docs/GUIDE.ru.md](https://github.com/89605502155/trosna-file/blob/main/docs/GUIDE.ru.md)).

| Время (Москва)      | temperature | status    |
|---------------------|-------------|-----------|
| 2026-10-08 10:00:00 | 21.5        | ok        |
| 2026-10-08 10:00:10 | 21.6        | ok        |
| 2026-10-08 10:00:20 | 21.7        | ok        |
| 2026-10-08 10:00:30 | 21.6        | door open |
| 2026-10-08 10:00:40 | 21.8        | ok        |

## Rust — [`rust/src/main.rs`](rust/src/main.rs)

Нужен Rust 1.89+ (<https://rustup.rs>). Библиотека подключается прямо из
GitHub (см. [`rust/Cargo.toml`](rust/Cargo.toml)).

```sh
cd rust
cargo run
```

Программа создаёт `temperature.trosna`, записывает пять точек одним коммитом,
затем открывает файл и печатает его содержимое и историю.

## Python — [`python/create_trosna.py`](python/create_trosna.py)

Нужны Python 3.11+ и Rust (пакет собирается из исходного кода).

```sh
cd python
pip install -r requirements.txt
python create_trosna.py
```

Скрипт сохраняет `pandas.DataFrame` в `temperature.trosna` одной строкой
`trosna.write(...)`, читает его обратно, исправляет одно значение и размечает
интервал, после чего показывает и текущую, и исходную версию данных.

## Командная строка — [`cli/demo.sh`](cli/demo.sh)

```sh
cargo install --git https://github.com/89605502155/trosna-file trosna-cli
sh cli/demo.sh
```

Скрипт превращает [`cli/temperature.csv`](cli/temperature.csv) в
`temperature.trosna` и показывает команды `info`, `cat`, `update`, `log`,
`diff` и `verify`.

## Готовые файлы

В папке [`samples/`](samples) лежат файлы, созданные этими примерами:
`temperature_rust.trosna` (один коммит) и `temperature_python.trosna` (два
коммита: исходные данные и исправление с меткой). Их можно открыть без
запуска примеров:

```sh
trosna cat samples/temperature_python.trosna --as-of 1
```

```python
import trosna
trosna.read_pandas("samples/temperature_python.trosna")
```

---

## English

Minimal examples that create a **`.trosna`** time-series file in Rust, in
Python and from the command line; all of them store the same hard-coded series
of five thermometer readings. The library lives at
<https://github.com/89605502155/trosna-file>.

* Rust: `cd rust && cargo run`
* Python: `cd python && pip install -r requirements.txt && python create_trosna.py`
* Command line: `cargo install --git https://github.com/89605502155/trosna-file trosna-cli && sh cli/demo.sh`

Licensed under either of Apache License 2.0 or MIT license at your option.

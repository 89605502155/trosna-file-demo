"""Creates temperature.trosna with a hard-coded time series of five points,
then opens the file again and prints what it contains.

Install:  pip install -r requirements.txt
Run:      python create_trosna.py
"""

import pandas as pd

import trosna

# 1. Five measurements of a room thermometer, one every 10 seconds.
series = pd.DataFrame(
    {
        "time": pd.date_range("2026-10-08 10:00:00", periods=5, freq="10s", tz="Europe/Moscow"),
        "temperature": [21.5, 21.6, 21.7, 21.6, 21.8],
        "status": ["ok", "ok", "ok", "door open", "ok"],
    }
)

# 2. Save it: one call creates temperature.trosna with the device "room1".
trosna.write("temperature.trosna", series, device="room1", message="five measurements", author="demo")

# 3. Read it back.
f = trosna.open("temperature.trosna")
print(f)
print(f.devices["room1"])
print(f.read_pandas())

# 4. Bonus: correct a value and label an interval; the old version stays readable.
with f.edit(message="sensor check", author="demo") as tx:
    tx.update("room1", "2026-10-08 10:00:20", temperature=21.65)
    tx.annotate("room1", "2026-10-08 10:00:25", "2026-10-08 10:00:35", "door open")
print(f.read_pandas())               # current data
print(f.read_pandas(as_of=1))        # as first written
for commit in f.commits():
    print(commit.number, commit.hash[:12], commit.message)

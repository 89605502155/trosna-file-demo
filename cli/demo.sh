#!/usr/bin/env sh
# Creates temperature.trosna from temperature.csv with the `trosna` tool and
# shows what the file contains.
# Install the tool first:
#   cargo install --git https://github.com/89605502155/trosna-file trosna-cli
set -e
cd "$(dirname "$0")"
rm -f temperature.trosna
trosna convert temperature.csv temperature.trosna --device room1 -m "five measurements"
trosna info temperature.trosna
trosna cat temperature.trosna --format table
trosna update temperature.trosna --time "2026-10-08 10:00:20" temperature=21.65 -m "sensor check"
trosna log temperature.trosna
trosna diff temperature.trosna --from 1
trosna verify temperature.trosna

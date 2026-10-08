//! Creates `temperature.trosna` with a hard-coded time series of five points,
//! then opens the file again and prints what it contains.
//!
//! Run: `cargo run` (in this `rust/` folder).

use trosna::{Batch, Column, CommitOptions, DataType, DeviceSchema, Reader, TimeUnit, Writer};

fn main() -> trosna::Result<()> {
    let path = "temperature.trosna";

    // 1. Five measurements of a room thermometer, one every 10 seconds,
    //    starting at 2026-10-08 10:00:00 Moscow time (07:00:00 UTC).
    //    Time stamps are integers: seconds since 1970-01-01 UTC.
    let t0: i64 = 1_791_442_800;
    let time: Vec<i64> = (0..5).map(|i| t0 + 10 * i).collect();
    let temperature = vec![21.5, 21.6, 21.7, 21.6, 21.8];
    let status = vec!["ok", "ok", "ok", "door open", "ok"];

    // 2. Create the file (replacing an old one) and describe the device:
    //    its time unit, time zone and value columns.
    if std::path::Path::new(path).exists() {
        std::fs::remove_file(path)?;
    }
    let mut writer = Writer::create(path)?;
    writer.create_device(
        DeviceSchema::new("room1", TimeUnit::Second)
            .with_timezone("Europe/Moscow")
            .with_column("temperature", DataType::Float64)
            .with_column("status", DataType::Utf8),
    )?;

    // 3. Write the five points and commit them; `close` finalizes the file.
    let batch = Batch::new(
        time,
        vec![
            ("temperature", Column::from(temperature)),
            ("status", Column::from(status)),
        ],
    )?;
    writer.write("room1", &batch)?;
    writer.commit_with(CommitOptions::message("five measurements").with_author("demo"))?;
    writer.close()?;

    // 4. Read the file back.
    let reader = Reader::open(path)?;
    let data = reader.read("room1")?;
    println!("{path}: {} bytes, {} points", reader.file_size(), data.len());
    let temperature = data.column("temperature").and_then(Column::as_f64).unwrap_or(&[]);
    let status = data.column("status").and_then(Column::as_utf8);
    for (i, t) in data.time().iter().enumerate() {
        let s = status.map_or("", |s| s.get(i));
        println!("  t = {t} s  temperature = {:>4}  status = {s}", temperature[i]);
    }
    for commit in reader.commits() {
        println!(
            "commit {} {} {:?}",
            commit.number,
            commit.short_hash(),
            commit.message.unwrap_or_default()
        );
    }
    Ok(())
}

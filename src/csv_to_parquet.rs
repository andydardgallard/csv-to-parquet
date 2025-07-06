use polars::prelude::*;

/// Represents raw CSV record read from input file.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CsvRecord {
    #[serde(rename = "<DATE>")]
    date: String,
    #[serde(rename = "<TIME>")]
    time: String,
    #[serde(rename = "<OPEN>")]
    open: f64,
    #[serde(rename = "<HIGH>")]
    high: f64,
    #[serde(rename = "<LOW>")]
    low: f64,
    #[serde(rename = "<CLOSE>")]
    close: f64,
    #[serde(rename = "<VOL>")]
    vol: u64,
}

/// Represents processed CSV record after formatting datetime.
#[derive(Debug, serde::Serialize)]
pub struct ProcessedRecord {
    datetime_str: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    vol: u64,
}

/// Reads CSV file, converts to Polars DataFrame, and writes as Parquet.
///
/// # Arguments
/// * `input_path` - Input CSV file path.
/// * `output_path` - Output Parquet file path.
///
/// # Returns
/// * `Result<()>` - Success or error if reading/writing fails.
pub fn convert_polars_to_parquet<P: AsRef<std::path::Path>>(input_dir_path: P, output_path: P) -> anyhow::Result<()> {
    let input_file = std::fs::File::open(input_dir_path)?;
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(input_file);
    let mut processed_records = Vec::new();

    for result in reader.deserialize() {
        let record: CsvRecord = result?;
        let date_str = &record.date;
        let time_str = &record.time;
        let dt_str = format!("{} {}", date_str, time_str);

        let dt = chrono::NaiveDateTime::parse_from_str(&dt_str, "%Y%m%d %H%M%S")
            .map_err(|e| anyhow::anyhow!("Failed to parse datetime: {}", e))?;

        let formatted_dt = dt.format("%Y-%m-%d %H:%M:%S").to_string();

        processed_records.push(ProcessedRecord {
            datetime_str: formatted_dt,
            open: record.open,
            high: record.high,
            low: record.low,
            close: record.close,
            vol: record.vol,
        });
    }  
    let df = csv_to_polars(&processed_records)?;

    let output_file = std::fs::File::create(output_path)?;
    polars::prelude::ParquetWriter::new(output_file).finish(&mut df.clone())?;
    // println!("{}", df);
    Ok(())
}

/// Converts CSV records into Polars DataFrame for Parquet serialization.
///
/// # Arguments
/// * `records` - Slice of processed CSV records.
///
/// # Returns
/// * `Result<DataFrame>` - Constructed DataFrame or error.
fn csv_to_polars(records: &[ProcessedRecord]) -> anyhow::Result<polars::frame::DataFrame> {
    let datetime_strs: Vec<&str> = records.iter().map(|r| r.datetime_str.as_str()).collect();
    let open: Vec<f64> = records.iter().map(|r| r.open).collect();
    let high: Vec<f64> = records.iter().map(|r| r.high).collect();
    let low: Vec<f64> = records.iter().map(|r| r.low).collect();
    let close: Vec<f64> = records.iter().map(|r| r.close).collect();
    let vol: Vec<u64> = records.iter().map(|r| r.vol).collect();

    let s_datetime = polars::prelude::Series::new("datetime_str", datetime_strs);
    let s_open = polars::prelude::Series::new("open", open);
    let s_high = polars::prelude::Series::new("high", high);
    let s_low = polars::prelude::Series::new("low", low);
    let s_close = polars::prelude::Series::new("close", close);
    let s_vol = polars::prelude::Series::new("vol", vol);

    let df = polars::frame::DataFrame::new(vec![s_datetime, s_open, s_high, s_low, s_close, s_vol])?;

    Ok(df)
}

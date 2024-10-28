// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use polars::prelude::*;

use RustQuant_error::RustQuantError;

/// Data struct.
/// Contains data format, the file path to the data, and the data itself.
pub struct Data {
    /// Data format.
    pub format: DataFormat,
    /// Data file path.
    pub path: String,
    /// Data (polars DataFrame).
    pub data: DataFrame,
}

/// Data format enum.
/// Currently supported formats are:
/// - CSV
/// - JSON
/// - PARQUET
pub enum DataFormat {
    /// CSV format.
    CSV,
    /// JSON format.
    JSON,
    /// PARQUET format.
    PARQUET,
}

/// Data reader trait.
/// Eagerly reads data from the source.
pub trait DataReader {
    /// Reads data from the source.
    fn read(&mut self) -> Result<(), RustQuantError>;
}

/// Data writer trait.
pub trait DataWriter {
    /// Writes data to the source.
    fn write(&mut self) -> Result<(), RustQuantError>;
}

/// Data scanner trait.
/// Lazily scans data from the source.
pub trait DataScanner {
    /// Scans data from the source.
    fn scan(&mut self) -> Result<LazyFrame, RustQuantError>;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Data {
    /// Creates a new `Data` struct.
    pub fn new(format: DataFormat, path: String) -> Self {
        Self {
            format,
            path,
            // Always creates an empty Polars DataFrame.
            data: DataFrame::default(),
        }
    }
}

impl DataReader for Data {
    fn read(&mut self) -> Result<(), RustQuantError> {
        match self.format {
            DataFormat::CSV => {
                // let df = CsvReader::from_path(&self.path)?.finish()?;

                let df = CsvReadOptions::default()
                    .try_into_reader_with_file_path(Some(self.path.clone().into()))?
                    .finish()?;

                self.data = df;

                Ok(())
            }
            DataFormat::JSON => {
                let mut file = std::fs::File::open(&self.path)?;
                let df = JsonReader::new(&mut file).finish()?;
                self.data = df;

                Ok(())
            }
            DataFormat::PARQUET => {
                let mut file = std::fs::File::open(&self.path)?;
                let df = ParquetReader::new(&mut file).finish()?;
                self.data = df;

                Ok(())
            }
        }
    }
}

impl DataWriter for Data {
    fn write(&mut self) -> Result<(), RustQuantError> {
        match self.format {
            DataFormat::CSV => {
                let mut file = std::fs::File::create(&self.path)?;

                CsvWriter::new(&mut file).finish(&mut self.data)?;

                Ok(())
            }
            DataFormat::JSON => {
                let mut file = std::fs::File::create(&self.path)?;

                JsonWriter::new(&mut file)
                    .with_json_format(JsonFormat::Json)
                    .finish(&mut self.data)?;

                Ok(())
            }
            DataFormat::PARQUET => {
                let mut file = std::fs::File::create(&self.path)?;

                ParquetWriter::new(&mut file).finish(&mut self.data)?;

                Ok(())
            }
        }
    }
}

impl DataScanner for Data {
    fn scan(&mut self) -> Result<LazyFrame, RustQuantError> {
        match self.format {
            DataFormat::CSV => Ok(LazyCsvReader::new(&self.path).finish()?),
            DataFormat::JSON => Ok(LazyJsonLineReader::new(self.path.clone()).finish()?),
            DataFormat::PARQUET => Ok(LazyFrame::scan_parquet(
                &self.path,
                ScanArgsParquet::default(),
            )?),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_io {
    use super::*;

    const READ_PATH: &str = "./src/examples/read";
    const WRITE_PATH: &str = "./src/examples/write";

    #[test]
    fn test_read_write_csv() -> Result<(), RustQuantError> {
        let mut data = Data {
            format: DataFormat::CSV,
            path: format!("{}.csv", READ_PATH), //String::from("./examples/example.csv"),
            data: DataFrame::default(),
        };

        data.read()?;

        data.path = format!("{}.csv", WRITE_PATH); //String::from("./examples/write.csv");

        data.write()?;

        println!("{:?}", data.data);

        Ok(())
    }

    #[test]
    fn test_read_write_json() -> Result<(), RustQuantError> {
        let mut data = Data {
            format: DataFormat::JSON,
            path: format!("{}.json", READ_PATH),
            data: DataFrame::default(),
        };

        data.read()?;

        data.path = format!("{}.json", WRITE_PATH);

        data.write()?;

        println!("{:?}", data.data);

        Ok(())
    }

    #[test]
    fn test_read_write_parquet() -> Result<(), RustQuantError> {
        let mut data = Data {
            format: DataFormat::PARQUET,
            path: format!("{}.parquet", READ_PATH),
            data: DataFrame::default(),
        };

        data.read()?;

        data.path = format!("{}.parquet", WRITE_PATH);

        data.write()?;

        println!("{:?}", data.data);

        Ok(())
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use polars::prelude::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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
    fn read(&mut self) -> Result<(), std::io::Error>;
}

/// Data writer trait.
pub trait DataWriter {
    /// Writes data to the source.
    fn write(&mut self) -> Result<(), std::io::Error>;
}

/// Data scanner trait.
/// Lazily scans data from the source.
pub trait DataScanner {
    /// Scans data from the source.
    fn scan(&mut self) -> Result<LazyFrame, std::io::Error>;
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
    fn read(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

/// Data writer trait.
pub trait DataWriter {
    /// Writes data to the source.
    fn write(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

/// Data scanner trait.
/// Lazily scans data from the source.
pub trait DataScanner {
    /// Scans data from the source.
    fn scan(&mut self) -> Result<LazyFrame, Box<dyn std::error::Error>>;
}

// ...

impl DataReader for Data {
    fn read(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.format {
            DataFormat::CSV => {
                let df = CsvReader::from_path(&self.path)?;
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
    fn write(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

                ParquetWriter::new(&mut file)
                    .finish(&mut self.data)?;

                Ok(())
            }
        }
    }
}

impl DataScanner for Data {
    fn scan(&mut self) -> Result<LazyFrame, Box<dyn std::error::Error>> {
        match self.format {
            DataFormat::CSV => Ok(LazyCsvReader::new(&self.path).finish()?),
            DataFormat::JSON => Ok(LazyJsonLineReader::new(self.path.clone()).finish()?),
            DataFormat::PARQUET => {
                Ok(LazyFrame::scan_parquet(&self.path, ScanArgsParquet::default())?)
            }
        }
    }
}



impl DataWriter for Data {
    fn write(&mut self) -> Result<(), std::io::Error> {
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

                ParquetWriter::new(&mut file)
                    .finish(&mut self.data)?;

                Ok(())
            }
        }
    }
}

impl DataScanner for Data {
    fn scan(&mut self) -> Result<LazyFrame, std::io::Error> {
        match self.format {
            DataFormat::CSV => Ok(LazyCsvReader::new(&self.path).finish())?,
            DataFormat::JSON => Ok(LazyJsonLineReader::new(self.path.clone()).finish())?,
            DataFormat::PARQUET => {
                Ok(LazyFrame::scan_parquet(&self.path, ScanArgsParquet::default()))?;
            }
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_io {
    use super::*;

    #[test]
    fn test_read_write_csv() {
        let mut data = Data {
            format: DataFormat::CSV,
            path: String::from("./src/data/examples/example.csv"),
            data: DataFrame::default(),
        };

        data.read()?;

        data.path = String::from("./src/data/examples/write.csv");

        data.write()?;

        println!("{:?}", data.data)
    }

    #[test]
    fn test_read_write_json() {
        let mut data = Data {
            format: DataFormat::JSON,
            path: String::from("./src/data/examples/example.json"),
            data: DataFrame::default(),
        };

        data.read()?;

        data.path = String::from("./src/data/examples/write.json");

        data.write()?;

        println!("{:?}", data.data)
    }

    #[test]
    fn test_read_write_parquet() {
        let mut data = Data {
            format: DataFormat::PARQUET,
            path: String::from("./src/data/examples/example.parquet"),
            data: DataFrame::default(),
        };

        data.read().unwrap();

        data.path = String::from("./src/data/examples/write.parquet");

        data.write().unwrap();

        println!("{:?}", data.data)
    }
}

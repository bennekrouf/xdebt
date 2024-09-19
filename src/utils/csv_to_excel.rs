
use calamine::{RangeDeserializerBuilder, Reader, Xlsx};
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

pub fn csv_to_excel(csv_file_path: &str, excel_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut workbook = calamine::Workbook::new();
    let mut sheet = workbook.add_worksheet(None)?;

    // Read CSV file
    let file = File::open(csv_file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    // Write CSV rows to the Excel sheet
    for (row_idx, result) in rdr.entries().enumerate() {
        let record = result?;
        for (col_idx, field) in record.iter().enumerate() {
            // Write cell values to the Excel file
            sheet.write_string(row_idx as u32, col_idx as u32, field)?;
        }
    }

    workbook.save(excel_file_path)?;

    Ok(())
}


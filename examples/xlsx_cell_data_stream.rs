// SPDX-License-Identifier: MIT
//
// Copyright 2016-2026, Johann Tuffe.

//! Demonstrates streaming the raw data of XLSX cells in one worksheet pass.
//!
//! The `XlsxCellData` record exposes all the information stored for a cell:
//! its position, style id, raw uninterpreted type/value and formula. The
//! style id and shared string indices resolve via the reader's `styles()`
//! and `shared_strings()` tables.
//!

use calamine::{open_workbook, RawCellType, Xlsx};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "tests/styles.xlsx";

    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let mut reader = workbook.worksheet_cells_reader("Sheet 1")?;

    // Stream the raw cell records in a single pass.
    while let Some(cell) = reader.next_cell_data()? {
        let (row, col) = cell.position();

        // Resolve shared string indices via the shared string table.
        let display = match cell.raw_type() {
            RawCellType::SharedString(index) => {
                format!(
                    "shared string {index} {:?}",
                    reader.shared_strings()[*index]
                )
            }
            raw_type => format!("{raw_type:?}"),
        };

        // Resolve the style id to a number format via the style palette.
        let style = &reader.styles()[cell.style_id()];
        let number_format = style
            .number_format
            .as_ref()
            .map_or("General", |format| format.format_code.as_str());

        println!(
            "({row}, {col}): {display}, style={}, format='{number_format}', formula={:?}",
            cell.style_id(),
            cell.formula(),
        );
    }

    Ok(())
}

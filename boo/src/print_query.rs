use super::LastResponse;
use bolt::error::BoltError;
use comfy_table::{Table, Cell, Row};

pub fn print_query_error(width: u16, error: BoltError) -> String {
    let table = get_error_table(width, error);
    let mut output = String::from("\r\n");

    output.push_str(&table.to_string().replace('\n', "\r\n"));
    output.push_str("\r\n");

    return output;
}

pub fn print_query_response(width: u16, result: &LastResponse, page_size: usize) -> String {
    let table = match get_result_table(width, result, page_size) {
        Some(t) => t,
        None => return String::from("\r\nEmpty response\r\n"),
    };

    let mut output = String::from("\r\n");

    output.push_str(&table.to_string().replace('\n', "\r\n"));
    output.push_str("\r\n");

    return output;
}

fn get_result_table(width: u16, result: &LastResponse, page_size: usize) -> Option<Table> {
    if result.size == 0 {
        return None;
    }

    let mut table = Table::new();

    table
        .load_preset(comfy_table::presets::UTF8_FULL)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_table_width(width);


    let response = &result.response;
    let mut row: Row = Row::new();
    row.add_cell(Cell::new("No."));

    table.set_header(
        response.fields().iter().fold(row, |mut row, field| {
            row.add_cell(Cell::new(field.to_string()));

            row
        })
    );

    for (index, result_row) in response.rows().iter().skip(result.index).take(page_size).enumerate() {
        let mut row = Row::new();
        row.add_cell(Cell::new(format!("{}", index + result.index)));

        table.add_row(result_row.iter().fold(row, |mut row, value| {
            row.add_cell(Cell::new(value.to_string()));

            row
        }));
    }

    return Some(table);
}

fn get_error_table(width: u16, error: BoltError) -> Table {
    let mut table = Table::new();

    table
        .load_preset(comfy_table::presets::UTF8_FULL)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_table_width(width);

    table.set_header(vec![Cell::new("Error")]);
    table.add_row(vec![Cell::new(format!("{}", error))]);

    return table;
}

/// Puts data into a table. The width of the table is at most the width of the terminal.
/// The final column is right-aligned, others are left-aligned. If the total width of the
/// table exceeds the width of the terminal, the longest column is truncated.
///
pub fn table(data: Vec<Vec<String>>) -> String {
    if data.is_empty() {
        return String::new();
    }

    let number_of_cols = data[0].len();
    let term_width = terminal_width();
    let mut widths: Vec<_> = (0..number_of_cols)
        .map(|i| longest_value_in_col(&data, i))
        .collect();
    let total_width = widths.iter().sum::<usize>() + 2 * number_of_cols;

    if total_width > term_width {
        let overspill = total_width - term_width;
        let truncate_field = field_to_truncate(&widths);
        widths[truncate_field] -= overspill;
    };

    data.into_iter()
        .map(|r| format_row(r, &widths))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_row(row: Vec<String>, widths: &[usize]) -> String {
    let rightmost = widths.len() - 1;

    row.into_iter()
        .enumerate()
        .map(|(i, cell)| {
            let width = widths[i] + 2;
            let cell_content = truncate_cell(cell, width);

            if i == rightmost {
                format!("{:>width$}", cell_content, width = width)
            } else {
                format!("{:<width$}", cell_content, width = width)
            }
        })
        .collect()
}

fn truncate_cell(mut cell_content: String, cell_width: usize) -> String {
    if cell_content.chars().count() > cell_width {
        cell_content.truncate(cell_width - 1);
        cell_content.push('\u{2026}');
    }
    cell_content
}

fn terminal_width() -> usize {
    terminal_size::terminal_size()
        .map(|(terminal_size::Width(w), _)| w as usize)
        .unwrap_or(80)
}

/// Gets the index of the widest field. If there's a tie it's the leftmost one
fn field_to_truncate(widths: &[usize]) -> usize {
    let mut widest_val = 0;
    let mut widest_field = 0;

    for (i, w) in widths.iter().enumerate() {
        if w > &widest_val {
            widest_field = i;
            widest_val = *w;
        }
    }

    widest_field
}

fn longest_value_in_col(tbl: &[Vec<String>], col: usize) -> usize {
    tbl.iter()
        .filter_map(|r| r.get(col))
        .map(|f| f.chars().count())
        .max()
        .unwrap_or(2)
        .to_owned()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_row() {
        assert_eq!(
            "08  of Montreal  Buried with Me                  Horse and Elephant Ea…",
            format_row(
                vec![
                    "08".to_owned(),
                    "of Montreal".to_owned(),
                    "Buried with Me".to_owned(),
                    "Horse and Elephant Eatery (No Elephants Allowed)".to_owned(),
                ],
                &[2, 11, 30, 20],
            )
        );
    }
}

use crate::input::TomlInput;
use dxf::entities::*;
use dxf::enums::HorizontalTextJustification;
use dxf::enums::VerticalTextJustification;
use dxf::tables::Style;
use dxf::Drawing;
use dxf::Point;
use std::error::Error;

fn add_lines(input: &TomlInput, drawing: &mut dxf::Drawing) {
    let num_rows = input.cells.len();
    let num_cols = input.cells[0].len();

    let table_width: f64 = input.cell_widths.iter().sum();
    let table_height = num_rows as f64 * input.cell_height;

    for i in 0..=num_rows {
        let p1 = Point {
            x: 0.0,
            y: -input.cell_height * i as f64,
            z: 0.0,
        };
        let p2 = Point {
            x: table_width,
            y: -input.cell_height * i as f64,
            z: 0.0,
        };
        let line = Line::new(p1, p2);
        let line = Entity::new(EntityType::Line(line));
        drawing.add_entity(line);
    }

    let mut x = 0.0;

    for j in 0..=num_cols {
        let p1 = Point { x, y: 0.0, z: 0.0 };
        let p2 = Point {
            x,
            y: -table_height,
            z: 0.0,
        };
        let line = Line::new(p1, p2);
        let line = Entity::new(EntityType::Line(line));
        drawing.add_entity(line);

        if j < num_cols {
            x += input.cell_widths[j];
        }
    }
}

fn add_texts(input: &TomlInput, drawing: &mut dxf::Drawing) {
    let num_rows = input.cells.len();
    let num_cols = input.cells[0].len();

    let style = Style {
        name: "my_style".to_string(),
        primary_font_file_name: input.primary_font_file_name.clone(),
        big_font_file_name: input.big_font_file_name.clone(),
        ..Default::default()
    };

    drawing.add_style(style);

    for i in 0..num_rows {
        let mut x = input.cell_widths[0] / 2.0;
        for j in 0..num_cols {
            x = if j == 0 {
                x
            } else {
                x + input.cell_widths[j - 1] / 2.0 + input.cell_widths[j] / 2.0
            };

            let dy = -input.cell_height;

            let y = dy * (i as f64 + 0.5);

            let text = Text {
                text_style_name: "my_style".to_string(),
                value: input.cells[i][j].clone(),
                location: Point::new(x, y, 0.0),
                text_height: input.text_height,
                relative_x_scale_factor: input.relative_x_scale_factor,
                horizontal_text_justification: HorizontalTextJustification::Center,
                second_alignment_point: Point::new(x, y, 0.0),
                vertical_text_justification: VerticalTextJustification::Middle,
                ..Default::default()
            };
            let text = Entity::new(EntityType::Text(text));
            drawing.add_entity(text);
        }
    }
}

pub fn write(input: TomlInput, output_file: String) -> Result<(), Box<dyn Error>> {
    let mut drawing = Drawing::new();

    add_lines(&input, &mut drawing);
    add_texts(&input, &mut drawing);

    drawing.save_file(output_file)?;

    Ok(())
}

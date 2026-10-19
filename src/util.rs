use std::time::Duration;

use crate::CieColor;

pub(crate) fn parse_cie_color_array(s: &str) -> Vec<CieColor> {
    s.split('}')
        .filter_map(|row| {
            let row = row.trim_start_matches('{').trim();
            if row.is_empty() { None } else { Some(row) }
        })
        .flat_map(|row| {
            row.split(',').filter_map(|num| {
                let num = num.trim();
                if num.is_empty() { None } else { num.parse::<f32>().ok() }
            })
        })
        .collect::<Vec<f32>>()
        .chunks(3)
        .map(|chunk| CieColor::new(chunk[0], chunk[1], chunk[2]))
        .collect()
}

pub(crate) fn parse_possibly_negative_duration(duration: f32) -> Duration {
    match duration {
        v if v >= 0.0 => Duration::from_secs_f32(v),
        _ => {
            eprintln!("Negative duration found");
            Duration::default()
        }
    }
}

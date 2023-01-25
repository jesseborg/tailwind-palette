use std::collections::{BTreeMap};
use serde_json::Value;

use crate::{colors::TAILWIND_COLORS, GeneratedColorFamily, Shade};

pub(crate) fn parse_tailwind_colors() -> Option<Vec<GeneratedColorFamily>> {
	let Ok(colors) = serde_json::from_str::<Value>(TAILWIND_COLORS) else {
		return None;
	};

	Some(colors
		.as_object()?
		.iter()
		.fold(Vec::new(), |mut tailwind_shades, (_, obj)| {
			let shades_map = obj
				.as_object()
				.unwrap()
				.iter()
				.fold(BTreeMap::new(), |mut acc, (key, value)| {
					acc.insert(key.parse::<u32>().unwrap(), value);
					acc
				});

			let shades = shades_map
				.iter()
				.fold(Vec::new(), |mut acc, (number, hexcode)| {
					let hex_str = hexcode.to_string();
					let hex = format!("#{}", &hex_str[1..hex_str.len() - 1]);

					acc.push(Shade {
						number: number.to_string(),
						hexcode: hex,
						delta: 0.0,
						lightness_diff: None
					});
					acc
				});

			tailwind_shades.push(GeneratedColorFamily {
				shades,
				closest_shade: None,
				closest_shade_lightness: None
			});
			tailwind_shades
		}))
}
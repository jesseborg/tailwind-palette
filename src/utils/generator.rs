use chroma_rust::Color;
use deltae::{DeltaE, LabValue as DeltaLabValue, DECMC1};

use crate::{GeneratedColorFamily, TailwindShade};

struct LabValue(DeltaLabValue);

impl From<&str> for LabValue {
	fn from(str: &str) -> Self {
		let (l, a, b) = Color::from(str).lab();
		// println!("Lab: {str} | {l} {a} {b}");
		Self(DeltaLabValue { l: l as f32, a: a as f32, b: b as f32 })
	}
}

pub(crate) fn generate_color_family(
	hexcode: String,
	reference_colors: Vec<GeneratedColorFamily>
) -> Option<Vec<TailwindShade>> {
	let Some(closest_color_family) = find_closest_color(
		&hexcode,
		reference_colors
	) else {
		return None;
	};

	let hexcode_color = Color::from(hexcode.as_str());
	let closest_color = Color::from(closest_color_family.closest_shade_lightness.as_ref()?.hexcode.as_str());

	// This needs to be done because of a bug in the 'chroma_rust' package
	// currently hue is not calculated correctly and falls below 0
	// when it should wrap 360
	let input_hue = match hexcode_color.hsl().0 {
		hue if hue < 0.0 => hue + 360.0,
		_ => hexcode_color.hsl().0
	};
	let match_hue = match closest_color.hsl().0 {
		hue if hue < 0.0 => hue + 360.0,
		_ => closest_color.hsl().0
	};

	let hue_difference = input_hue - match_hue;
	let saturation_ratio = hexcode_color.hsl().1 / closest_color.hsl().1;

	Some(closest_color_family
		.shades
		.iter()
		.map(|shade| {
			let new_color_hsl = Color::from(shade.hexcode.as_str()).hsl();
			
			let new_color_hsl = match new_color_hsl.0 {
				hue if hue < 0.0 => (hue + 360.0, new_color_hsl.1, new_color_hsl.2),
				_ => new_color_hsl
			};
			let new_saturation = new_color_hsl.1 * saturation_ratio;

			let mut new_hue = match hue_difference {
				d if d < 0.0 => new_color_hsl.0 + d,
				d if d > 0.0 => new_color_hsl.0 + d,
				_ => match_hue
			};

			new_hue = match new_hue {
				hue if hue < 0.0 => hue + 360.0,
				hue if hue > 360.0 => hue - 360.0,
				_ => new_hue,
			};

			// println!("{:?}", format!("hsl({},{},{})", new_hue, new_saturation, new_color_hsl.2));

			let mut new_color_hex = Color::from(
				format!("hsl({},{},{})", new_hue, new_saturation, new_color_hsl.2).as_str()
			);

			if let Some(closest_shade) = closest_color_family.closest_shade_lightness.as_ref() {
				if closest_shade.number == shade.number {
					new_color_hex = Color::from(hexcode.as_str());
				}
			}

			TailwindShade {
				number: shade.number.clone(),
				hexcode: new_color_hex.hex(),
				rgb: new_color_hex.rgb()
			}
		}
	).collect())
}

fn find_closest_color(
	hexcode: &String,
	mut reference_colors: Vec<GeneratedColorFamily>
) -> Option<GeneratedColorFamily> {
	reference_colors
		.iter_mut()
		.for_each(|color_family| {
			// println!("family: {:#?}", color_family);
			// Add DELTA key / value to array of objects
			color_family
				.shades
				.iter_mut()
				.for_each(|shade| {
					shade.delta = *DeltaE::new(
						LabValue::from(hexcode.as_str()).0, 
						LabValue::from(shade.hexcode.as_str()).0, 
						DECMC1
					).value();
				});
	
			color_family.closest_shade = color_family
				.shades
				.iter()
				.min_by_key(|shade| shade.delta as u32)
				.cloned();
		});

	// Calculate the color family with the lowest DELTA
	let Some(mut closest_color_family) = reference_colors
		.iter()
		.reduce(|previous, current| {
			let Some(prev_closest_shade) = previous.closest_shade.as_ref() else {
				return previous;
			};
			
			let Some(cur_closest_shade) = current.closest_shade.as_ref() else {
				return previous;
			};

			if prev_closest_shade.delta < cur_closest_shade.delta {
				return previous;
			}

			current
		}).and_then(|color_family| Some(color_family.to_owned())) else {
			return None;
		};

	closest_color_family
		.shades
		.iter_mut()
		.for_each(|shade| {
			shade.lightness_diff = Some(
				(Color::from(shade.hexcode.as_str()).hsl().2 - Color::from(hexcode.as_str()).hsl().2).abs()
			);
		});

	closest_color_family.closest_shade_lightness = closest_color_family
		.shades
		.iter()
		.reduce(|previous, current| {
			if previous.lightness_diff < current.lightness_diff {
				return previous;
			}
			current
		})
		.cloned();

	Some(closest_color_family)
}
#![feature(test)]

extern crate test;

use chroma_rust::Color;
use rspc::Type;
use serde::{Deserialize, Serialize};
use utils::{colors::parse_tailwind_colors, generator::generate_color_family};

mod colors;
mod utils;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Type)]
#[allow(dead_code)]
pub struct TailwindShade {
    number: String,
    hexcode: String,
    rgb: (u8, u8, u8),
}

impl TailwindShade {
    pub fn number(&self) -> &str {
        &self.hexcode.as_str()
    }

    pub fn hex(&self) -> &str {
        &self.hexcode.as_str()
    }

		pub fn rgb(&self) -> (u8,u8,u8) { self.rgb }
}

#[derive(Debug, Clone)]
pub struct Shade {
    number: String,
    hexcode: String,
    delta: f32,
    lightness_diff: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct GeneratedColorFamily {
    shades: Vec<Shade>,
    closest_shade: Option<Shade>,
    closest_shade_lightness: Option<Shade>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Type)]
pub struct TailwindPalette {
    shades: Vec<TailwindShade>,
}

impl TailwindPalette {
    pub fn new(str: &str) -> Option<Self> {
        let Some(reference_colors) = parse_tailwind_colors() else {
			println!("Error parsing reference colors");
			return None;
		};

        let Some(shades) = generate_color_family(
			Color::from(str).hex(),
			reference_colors
		) else {
			println!("Could not generate a palette");
			return None;
		};

        Some(Self { shades })
    }

    pub fn shades(&self) -> &Vec<TailwindShade> {
        &self.shades
    }

    pub fn shades_as_svg(&self) -> String {
        if self.shades.len() == 0 {
            return "".into();
        }

        let mut svg = "<svg width='1000' height='100' viewBox='0 0 1000 100' fill='none' xmlns='http://www.w3.org/2000/svg'>\n".to_string();
        for (i, shade) in self.shades.iter().enumerate() {
            svg.push_str(
                format!(
                    "  <rect x='{:?}' width='100' height='100' fill='{}'/>\n",
                    i * 100,
                    shade.hex()
                )
                .as_str(),
            );
        }
        svg.push_str("</svg>");

        svg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn rgb_238() {
        let palette = TailwindPalette::new("#8c8c8c");
        assert_eq!(true, palette.is_some());
    }

    #[bench]
    fn test(bencher: &mut Bencher) {
        bencher.iter(|| {
            for x in 0..139 {
                // println!("{}", x);
                TailwindPalette::new(format!("rgb({},{},{})", x % 255, x % 255, x % 255).as_str());
            }
        });
    }

    #[bench]
    fn generate_colors_test(bencher: &mut Bencher) {
        bencher.iter(|| {
            for _ in 0..30 {
                parse_tailwind_colors();
            }
        });
    }
}

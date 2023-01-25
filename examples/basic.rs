use tailwind_palette::TailwindPalette;

fn main() {
    let palette = TailwindPalette::new("#a788ee");
    println!("{}", palette.as_ref().unwrap().shades_as_svg());
}

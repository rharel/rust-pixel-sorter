#[macro_use]
extern crate clap;
extern crate image;
extern crate palette;

mod color;
mod command_line_interface;
mod program_arguments;
mod sorting;

use command_line_interface::parse_arguments;
use image::{DynamicImage, RgbImage};
use program_arguments::ProgramArguments;
use sorting::{SortingOptions, sort_in_place};
use std::fs::File;
use std::process::exit;

fn load_input(file_path: &str, is_verbose: bool) -> RgbImage
{
    if is_verbose { println!("Loading image at {}", file_path); }
    let image = image::open(file_path).unwrap_or_else(|_|
    {
        println!("ERROR: Failed to load input image.");
        exit(1);
    });
    return match image
    {
        DynamicImage::ImageRgb8(buffer) => buffer,
        _ => image.to_rgb()
    };
}
fn sort(
    image: &mut RgbImage,
    options: &SortingOptions,
    is_verbose: bool)
{
    if is_verbose
    {
        println!("Sorting by component: {:?}", options.comparison_component);
        println!("Sorting along dimension: {:?}", options.dimension);
        println!("Affected components: {:?}", options.affected_components);
    }
    sort_in_place(image, options);
}
fn save_output(
    image: &DynamicImage,
    file_path: &str,
    is_verbose: bool)
{
    if is_verbose { println!("Saving output to {}", file_path); }

    let ref mut file = File::create(file_path).unwrap_or_else(|_|
    {
        println!("ERROR: Failed to open output file.");
        exit(1);
    });
    image.save(file, image::PNG).unwrap_or_else(|_|
    {
        println!("ERROR: Failed to save output image.");
        exit(1);
    });
    println!("Output was saved to {}", file_path);
}
fn main()
{
    let ProgramArguments
    {
        input_file_path,
        output_path,
        comparison_component,
        dimension,
        affected_components,
        is_verbose
    } = parse_arguments();

    let mut image = load_input(input_file_path.as_str(), is_verbose);
    let sorting_options = SortingOptions
    {
        comparison_component,
        dimension,
        affected_components
    };
    sort(&mut image, &sorting_options, is_verbose);
    save_output(
        &DynamicImage::ImageRgb8(image),
        output_path.as_str(),
        is_verbose
    );
}

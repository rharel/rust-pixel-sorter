use clap::App;
use color::{ColorComponent, ColorSpaceMask};
use program_arguments::ProgramArguments;
use sorting::Dimension;
use std::str::FromStr;

/// Parses command line arguments.
pub fn parse_arguments() -> ProgramArguments
{
    let cli_yaml = load_yaml!("command_line_interface.yml");
    let matches = App::from_yaml(cli_yaml).get_matches();
    ProgramArguments
    {
        input_file_path: String::from(matches.value_of("INPUT").unwrap()),
        output_path:     String::from(matches.value_of("output").unwrap()),

        comparison_component: ColorComponent::from_str(
            matches.value_of("comparison_component").unwrap()
        ).unwrap(),
        dimension: Dimension::from_str(
            matches.value_of("sorting_dimension").unwrap()
        ).unwrap(),
        affected_components:
        {
            let mut mask: ColorSpaceMask = Default::default();
            for argument in vec!["affect_hsl", "affect_hsv", "affect_rgb"]
            {
                if let Some(mut values) = matches.values_of(argument)
                {
                    mask = ColorSpaceMask::from([
                        values.next().unwrap(),
                        values.next().unwrap_or(""),
                        values.next().unwrap_or("")
                    ]);
                }
            }
            mask
        },

        is_verbose: matches.is_present("verbose")
    }
}

This is a simple command-line application for applying some pixel-sorting effects to an image. You can find some sample output images under the [demo directory](demo/).

## Usage
```
USAGE:
    pixelsort.exe [FLAGS] [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Sets program verbosity level.

OPTIONS:
        --affect-hsl <affect_hsl>           Sets the components to be affected in HSL space. [values: Hue, Saturation,
                                            Lightness]
        --affect-hsv <affect_hsv>           Sets the components to be affected in HSV space. [values: Hue, Saturation,
                                            Value]
        --affect-rgb <affect_rgb>           Sets the components to be affected in RGB space. [values: Red, Green, Blue]
    -c, --compare <comparison_component>    Sets the color component to sort by. [default: Red]  [values: Red, Green,
                                            Blue, Hue, Saturation, Lightness, Value]
    -o, --output <output>                   Sets the output file path. [default: ./output.png]
    -d, --dimension <sorting_dimension>     Sets the sorting dimension. [default: Row]  [values: Row, Column]

ARGS:
    <INPUT>    Sets the input image file.
```
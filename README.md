A program that preprocesses image files with Rust and combines them into a pdf with imagemagick. Supported image types depend on https://crates.io/crates/image#supported-image-formats.

```
Usage: pdfify [OPTIONS] --files [<FILES>...]

Options:
  -f, --files [<FILES>...]   image files to pdfify
  -o, --output <OUTPUT>      output file name
  -v, --verbose              show verbose output
  -d, --dimages              delete both the processed and original images after pdfifying
      --doriginal            delete only the original images after pdfifying
      --dprocessed           delete only the processed images after pdfifying
  -b, --brighten <BRIGHTEN>  adjust brightness by a custom amount (integer amount)
  -c, --contrast <CONTRAST>  adjust contrast by a custom amount (-100.0 to 100.0 percent)
  -h, --help                 Print help
  -V, --version              Print version
```
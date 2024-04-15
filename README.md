# pdfify

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

As a student, I often take pictures of my work and upload them to websites like GradeScope for evaluation. The GradeScope mobile app would be great if it didn't constantly crash on my Android phone. This is meant to be a substitute for the first alternative I found, [Openscan](https://github.com/ethereal-developers/OpenScan), because that app seems to shrink images before they are processed into a clean pdf. 

## Installation

Requires imagemagick's `convert`. Old versions of GhostScript, a pdf conversion dependency, appear to have a security flaw. So, converting images to pdfs is disabled by default. Refer to [this post](https://askubuntu.com/questions/1127260/imagemagick-convert-not-allowed) to renable the feature.

On a system with Rust [installed](https://doc.rust-lang.org/book/ch01-01-installation.html) (which includes its `cargo` package manager), run 
```cargo install --git https://github.com/wade-cheng/pdfify```.
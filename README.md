<div align="center" id="top"> 
  <img src="./.github/app.gif" alt="Label_studio_yolo_datasets_converter" />

  &#xa0;

  <!-- <a href="https://label_studio_yolo_datasets_converter.netlify.app">Demo</a> -->
</div>

<h1 align="center">Label_studio_yolo_datasets_converter</h1>

<p align="center">
  <img alt="Github top language" src="https://img.shields.io/github/languages/top/xsxz01/label_studio_yolo_datasets_converter?color=56BEB8">

  <img alt="Github language count" src="https://img.shields.io/github/languages/count/xsxz01/label_studio_yolo_datasets_converter?color=56BEB8">

  <img alt="Repository size" src="https://img.shields.io/github/repo-size/xsxz01/label_studio_yolo_datasets_converter?color=56BEB8">

  <img alt="License" src="https://img.shields.io/github/license/xsxz01/label_studio_yolo_datasets_converter?color=56BEB8">

  <img alt="Github issues" src="https://img.shields.io/github/issues/xsxz01/label_studio_yolo_datasets_converter?color=56BEB8" />

  <img alt="Github forks" src="https://img.shields.io/github/forks/xsxz01/label_studio_yolo_datasets_converter?color=56BEB8" />

  <img alt="Github stars" src="https://img.shields.io/github/stars/xsxz01/label_studio_yolo_datasets_converter?color=56BEB8" />
</p>

<!-- Status -->

<h4 align="center"> 
	🚧  Label_studio_yolo_datasets_converter 🚀 Under construction...  🚧
</h4> 

<hr>

<p align="center">
  <a href="#dart-about">About</a> &#xa0; | &#xa0; 
  <a href="#sparkles-features">Features</a> &#xa0; | &#xa0;
  <a href="#rocket-technologies">Technologies</a> &#xa0; | &#xa0;
  <a href="#white_check_mark-requirements">Requirements</a> &#xa0; | &#xa0;
  <a href="#checkered_flag-starting">Starting</a> &#xa0; | &#xa0;
  <a href="#memo-license">License</a> &#xa0; | &#xa0;
  <a href="https://github.com/xsxz01" target="_blank">Author</a>
</p>

<br>

## :dart: About ##

A tool to convert the YOLO formatted annotation dataset exported from Label Studio into a YOLO trainable annotation dataset format.

## :sparkles: Features ##

:heavy_check_mark: Written purely in Rust;\
:heavy_check_mark: Supports YOLO format exported from Label Studio;\
:heavy_check_mark: Supports direct ZIP conversion or specifying a folder;

## :rocket: Technologies ##

The following tools were used in this project:

- [Rust](https://www.rust-lang.org/)
- [clap](https://crates.io/crates/clap)
- [zip](https://crates.io/crates/zip)

## :white_check_mark: Requirements ##

Before starting :checkered_flag:, you need to have [Rust](https://www.rust-lang.org/) installed.

## :checkered_flag: Starting ##

```bash
# Clone this project
$ git clone https://github.com/xsxz01/label_studio_yolo_datasets_converter

# Access
$ cd label_studio_yolo_datasets_converter

# Install by source
$ cargo install --path .

# Run the cli
$ lsdc -o output_folder <method> <input_path>

# Get help
$ lsdc --help

# Get version
$ lsdc --version
```

## :memo: License ##

This project is under license from MIT. For more details, see the [LICENSE](LICENSE) file.


Made with :heart: by <a href="https://github.com/xsxz01" target="_blank">Pang</a>

&#xa0;

<a href="#top">Back to top</a>

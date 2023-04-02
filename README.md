# image-label-tool

[![Github Repo](https://img.shields.io/badge/github-repo-green)](https://github.com/chriamue/image-label-tool/)
[![Github Pages Build](https://github.com/chriamue/image-label-tool/actions/workflows/gh-pages.yml/badge.svg)](https://chriamue.github.io/image-label-tool/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/chriamue/image-label-tool/branch/main/graph/badge.svg?token=TFJ8UT9W1J)](https://codecov.io/gh/chriamue/image-label-tool)
[![Demo](https://img.shields.io/badge/Demo-online-green.svg)](https://chriamue.github.io/image-label-tool/)
[![Doc](https://img.shields.io/badge/Docs-online-green.svg)](https://chriamue.github.io/image-label-tool/image_label_tool/)

Create bounding boxes around objects in an image and classify them according to your desired labels.

## About

This tool is written in Rust and compiled to WebAssembly (Wasm) for use in the browser. It allows you to create bounding boxes around objects in an image and classify them according to your desired labels.

To use the tool, simply open it in your browser and upload an image. You can then draw bounding boxes around the objects you want to classify by clicking and dragging your mouse.

Once you have labeled all the objects in the image, you can export the results. The file will contain the coordinates and labels for each bounding box.

Thank you for using our label tool! I hope it helps with your machine learning projects.

## Data Format

Exported data will be stored in Yolo format.

```txt
<object-class> <x> <y> <width> <height>
```

## Usage

1. Clone the repository:

    ```sh
    git clone https://github.com/chriamue/image-label-tool
    cd image-label-tool
    ```

2. Compile the code to WASM:

    ```sh
    wasm-pack build --target web
    ```

3. Run the Web version in your browser

    ```sh
    python3 -m http.server
    ```

Open your browser on [Localhost](http://localhost:8000)

# 🫧🧽 Photo File Sorter

## From unorganised photos to organised folders

### Before

<img src="assets/스크린샷 2024-03-29 173940.png" width="500">

### After

<img src="assets/스크린샷 2024-03-29 174337.png" width="500">

This is a mini Rust project that sorts photos based on their creation date. It reads the Exif data of the photos and sorts them into folders based on the year and month they were taken. Currently, it supports .JPG, .JPEG, .RAF and .MOV files.

## Video Demo

https://github.com/denim-bluu/photo_file_sorter/assets/66572804/04500a33-27b5-4960-a1ff-d000f5b36fdd

## Getting Started

### Disclaimer

Currently, this tool is pretty basic and only supports .JPG, .JPEG, .RAF and .MOV files. I'm **NOT** planning to add support for other file types or videos. This is just a mini project I made for fun. Also, this tool will not be responsible for any data loss or corruption. Use it at your own risk.

### Installing

1. Clone the repository

    ```sh
    git clone https://github.com/yourusername/photo_file_sorter.git
    ```

2. Navigate into the cloned repository

    ```sh
    cd photo_file_sorter
    ```

3. Build the project

    ```sh
    cargo build
    ```

## Usage

### Running rust code

To run the project, use the following command:

```sh
cargo run
```

### Creating & Running the Windows executable

If you want an `.exe` file, you can build the project using the following command:

```sh
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

`.exe` file will be located in `target/x86_64-pc-windows-gnu/release/` and you need to place the `.exe` file in the directory you want to sort the photos.

## Built With

- Rust - The programming language used
- Chrono - A date and time library for Rust
- Rexif - A library to parse Exif data
- Walkdir - A library to walk directories

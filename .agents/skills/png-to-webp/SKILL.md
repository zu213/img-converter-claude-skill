---
name: png-to-webp
description: Convert PNG images to WebP format using a hand-rolled Rust encoder. Use this skill whenever the user asks to convert PNG to WebP, compress PNG images, optimize images for the web, batch-convert PNGs, or do any image format conversion involving PNG or WebP. Also trigger when the user mentions reducing image file size, preparing images for web deployment, or converting assets to modern formats. Even if the user just says "make this image smaller" or "optimize this image" and it's a PNG, use this skill.
---

# PNG to WebP Converter

A custom Rust-based PNG → lossy WebP encoder.

## Prerequisites

Rust toolchain must be installed. If not present:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Setup (first run only)

The skill bundles a Rust project. Build it before first use:

```bash
cd <skill-directory>/tool
cargo build --release
```

The binary is at `tool/target/release/png-to-webp`.

## Converting a single file

```bash
./tool/target/release/png-to-webp INPUT.png -q QUALITY -o OUTPUT.webp
```

- `INPUT.png` — path to the source PNG (required)
- `-q QUALITY` — encoding quality 0–100 (optional, default 80)
- `-o OUTPUT.webp` — output path (optional, defaults to input name with `.webp` extension)

## Batch conversion

For converting a directory of PNGs, write a loop:

```bash
for f in /path/to/images/*.png; do
  ./tool/target/release/png-to-webp "$f" -q 80
done
```

## Choosing quality

Pick quality based on the use case:

| Use case              | Quality | Notes                          |
|-----------------------|---------|--------------------------------|
| Photography / print   | 90–100  | Near-lossless, larger files    |
| General web images    | 75–85   | Good default, start with 80   |
| Thumbnails / previews | 50–70   | Smaller, some artifacts OK     |
| Maximum compression   | 20–49   | Visible loss, tiny files       |

When the user doesn't specify quality, default to 80.

## Error handling

- If the input file doesn't exist or isn't a valid PNG, the tool exits with an error message.
- If quality is out of range, it prints a warning and exits.
- If the output path's directory doesn't exist, create it first with `mkdir -p`.

## Limitations

- Input must be PNG (other formats are not supported).
- Output is always lossy WebP (no lossless mode yet).
- Transparency (alpha channel) is not preserved — alpha is dropped during YUV conversion.
- Output files will be larger than libwebp for the same quality setting (the encoder is simplified).

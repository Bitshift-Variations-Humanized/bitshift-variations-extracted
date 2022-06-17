# bitshift-variations-extracted
Rust program to Extract Robert Miles's "Bitshift Variations in C Minor" as raw audio from its codegolfy glory

It works by reading the stdout of the [original executable](https://txti.es/bitshiftvariationsincminor), and storing an initial slice of data. It then proceeds to scan a window of the output stream until it detects the original slice of data repeating.
It then proceeds to output the raw audio and the `.wav` file.

`cargo run` to run.

See the [project readme](https://github.com/Bitshift-Variations-Humanized/,github) for more details

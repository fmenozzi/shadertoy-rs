Desktop [Shadertoy](https://www.shadertoy.com) client, written in Rust.

While it's still very much a work in progress, you can run some example shaders to see that it's working:

```
cargo run --example seascape
cargo run --example elemental-ring
```

For now, the CLI looks like this:

```
$ shadertoy-rs --help
shadertoy-rs 0.1.0
Federico Menozzi <federicogmenozzi@gmail.com>
Desktop client for Shadertoy

USAGE:
    shadertoy-rs [FLAGS] [OPTIONS] [shader]

FLAGS:
    -h, --help                  Prints help information
    -n, --not-from-shadertoy    For shaders not copy-pasted from Shadertoy
    -V, --version               Prints version information

OPTIONS:
    -H, --height <height>    Sets window height [default: 400]
    -W, --width <width>      Sets window width [default: 600]

ARGS:
    <shader>    Path to fragment shader [default: shaders/default.frag]
````

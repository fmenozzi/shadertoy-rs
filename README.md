Desktop [Shadertoy](https://www.shadertoy.com) client, written in Rust.

While it's still very much a work in progress, you can run some example shaders to see that it's working:

```
cargo run --example seascape
cargo run --example elemental-ring
```

You can press `F5` to reload the shader if you've edited it since launching the app.

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
    -H, --height <height>        Sets window height [default: 400]
        --texture0 <texture0>    Path to 2D RGBA texture for iChannel0 [default: textures/brickwall.jpg]
        --texture1 <texture1>    Path to 2D RGBA texture for iChannel1 [default: textures/landscape.jpg]
        --texture2 <texture2>    Path to 2D RGBA texture for iChannel2 [default: textures/whitenoise.jpg]
        --texture3 <texture3>    Path to 2D RGBA texture for iChannel3 [default: textures/woodgrain.jpg]
    -W, --width <width>          Sets window width [default: 600]

ARGS:
    <shader>    Path to fragment shader [default: shaders/default.frag]
````

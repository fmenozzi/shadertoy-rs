Desktop [Shadertoy](https://www.shadertoy.com) client, written in Rust.

While it's still a work in progress, you can run some example shaders to see that it's working:

```
cargo run --release -- --example seascape
cargo run --release -- --example elemental-ring
```

Make sure you build/run in release mode; textures take several seconds to load in debug mode.

So long as you restrict yourself to the supported uniforms, shaders copy-pasted directly from Shadertoy should run with no modifications. The following uniforms are currently supported, with more coming soon:

* `iGlobalTime`
* `iResolution`
* `iMouse`
* `iFrame`
* `iChannel0`, `iChannel1`, `iChannel2`, `iChannel3`
    * These are 2D RGBA textures

You can press `F5` to reload the shader if you've edited it since launching the app.

For now, the CLI looks like this:

```
shadertoy 0.1.1
Federico Menozzi <federicogmenozzi@gmail.com>
Desktop client for Shadertoy

USAGE:
    shadertoy [OPTIONS] [shader]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --example <example>      Run example shader from examples/ directory
    -H, --height <height>        Sets window height [default: 400]
        --texture0 <texture0>    Path to 2D RGBA texture for iChannel0
        --texture1 <texture1>    Path to 2D RGBA texture for iChannel1
        --texture2 <texture2>    Path to 2D RGBA texture for iChannel2
        --texture3 <texture3>    Path to 2D RGBA texture for iChannel3
    -W, --width <width>          Sets window width [default: 600]

ARGS:
    <shader>    Path to fragment shader [default: shaders/default.frag]
````

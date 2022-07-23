use error;
use gfx::texture::{FilterMethod, WrapMode};

use clap::App;

pub struct ArgValues {
    pub width: f32,
    pub height: f32,

    // Path to the shader. None if using default fragment shader.
    pub shaderpath: Option<String>,

    // Path to the n-th texture. None if using default textures.
    pub texture0path: Option<String>,
    pub texture1path: Option<String>,
    pub texture2path: Option<String>,
    pub texture3path: Option<String>,

    // Wrap mode for the n-th texture. Defaults to "clamp" if unspecified.
    pub wrap0: WrapMode,
    pub wrap1: WrapMode,
    pub wrap2: WrapMode,
    pub wrap3: WrapMode,

    // Filter method for the n-th texture. Defaults to "mipmap" if unspecified.
    pub filter0: FilterMethod,
    pub filter1: FilterMethod,
    pub filter2: FilterMethod,
    pub filter3: FilterMethod,

    // Max value for anisotropic filtering. Defaults to 1 if unspecified. Only needed for
    // "anisotropic" filter method.
    pub anisotropic_max: u8,

    // Some(name) if running an example.
    pub examplename: Option<String>,

    // Shadertoy id if downloading a shader.
    pub getid: Option<String>,

    // Custom window title. Defaults to "{shader name} - shadertoy-rs".
    pub title: Option<String>,

    // True if also running downloaded shader.
    pub andrun: bool,
}

impl ArgValues {
    pub fn from_cli() -> error::Result<ArgValues> {
        // Load CLI matches.
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();

        // Closure for converting &str to String.
        let str_to_string = |s: &str| s.to_string();

        // Window dimensions.
        let width = matches.value_of("width").unwrap().parse()?;
        let height = matches.value_of("height").unwrap().parse()?;

        // Check to see if they want an example run.
        let examplename = matches.value_of("example").map(&str_to_string);

        // Fragment shader path.
        let shaderpath = matches.value_of("shader").map(&str_to_string);

        // Texture paths.
        let texture0path = matches.value_of("texture0").map(&str_to_string);
        let texture1path = matches.value_of("texture1").map(&str_to_string);
        let texture2path = matches.value_of("texture2").map(&str_to_string);
        let texture3path = matches.value_of("texture3").map(&str_to_string);

        // Texture wrapping.
        let get_wrap_mode = |wrap_mode: &Option<&str>| match wrap_mode.unwrap_or("clamp") {
            "clamp" => WrapMode::Clamp,
            "repeat" => WrapMode::Tile,
            "mirror" => WrapMode::Mirror,
            "border" => WrapMode::Border,
            _ => WrapMode::Clamp,
        };
        let wrap0 = get_wrap_mode(&matches.value_of("wrap0"));
        let wrap1 = get_wrap_mode(&matches.value_of("wrap1"));
        let wrap2 = get_wrap_mode(&matches.value_of("wrap2"));
        let wrap3 = get_wrap_mode(&matches.value_of("wrap3"));

        // Anistropic filter max value.
        let anisotropic_max = matches
            .value_of("anisotropic_max")
            .unwrap_or("1")
            .parse::<u8>()
            .unwrap_or(1)
            .clamp(1, 16);

        // Texture filtering.
        let get_filter_mode = |filter_mode: &Option<&str>| match filter_mode.unwrap_or("mipmap") {
            "scale" => FilterMethod::Scale,
            "mipmap" => FilterMethod::Mipmap,
            "bilinear" => FilterMethod::Bilinear,
            "trilinear" => FilterMethod::Trilinear,
            "anisotropic" => FilterMethod::Anisotropic(anisotropic_max),
            _ => FilterMethod::Mipmap,
        };
        let filter0 = get_filter_mode(&matches.value_of("filter0"));
        let filter1 = get_filter_mode(&matches.value_of("filter1"));
        let filter2 = get_filter_mode(&matches.value_of("filter2"));
        let filter3 = get_filter_mode(&matches.value_of("filter3"));

        // Window title.
        let title = matches.value_of("title").map(&str_to_string);

        // Check to see if they want to download a shader (and then run it).
        let (getid, andrun) = if let Some(getmatches) = matches.subcommand_matches("get") {
            (
                getmatches.value_of("id").map(&str_to_string),
                getmatches.is_present("run"),
            )
        } else {
            (None, false)
        };

        Ok(ArgValues {
            width,
            height,
            shaderpath,
            texture0path,
            texture1path,
            texture2path,
            texture3path,
            wrap0,
            wrap1,
            wrap2,
            wrap3,
            filter0,
            filter1,
            filter2,
            filter3,
            anisotropic_max,
            examplename,
            getid,
            andrun,
            title,
        })
    }
}

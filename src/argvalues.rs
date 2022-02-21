use error;
use gfx::texture::{WrapMode, FilterMethod};

use clap::App;

pub struct ArgValues {
    pub width: f32,
    pub height: f32,

    // None if using default fragment shader
    pub shaderpath: Option<String>,

    // None if using default textures
    pub texture0path: Option<String>,
    pub texture1path: Option<String>,
    pub texture2path: Option<String>,
    pub texture3path: Option<String>,

    // Texture wrapping
    pub wrap0: Option<WrapMode>,
    pub wrap1: Option<WrapMode>,
    pub wrap2: Option<WrapMode>,
    pub wrap3: Option<WrapMode>,

    // Filter method
    pub filter0: Option<FilterMethod>,
    pub filter1: Option<FilterMethod>,
    pub filter2: Option<FilterMethod>,
    pub filter3: Option<FilterMethod>,

    // Max value for anisotropic filtering
    pub anisotropic_max: Option<u8>,

    // Some(name) if running an example
    pub examplename: Option<String>,

    // Some(id) if downloading a shader
    pub getid: Option<String>,

    // a custom window title
    pub title: Option<String>,

    // true if also running downloaded shader
    pub andrun: bool,
}

impl ArgValues {
    pub fn from_cli() -> error::Result<ArgValues> {
        // Load CLI matches
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();

        // Closure for converting &str to String
        let str_to_string = |s: &str| s.to_string();

        // Convert &str to integer between 1 and 16
        fn str_to_anisotropic_max(s: &str) -> u8 {
            match s.parse::<u8>() {
                Ok(i) => i.clamp(1,16),
                Err(_e) => 1,
            }
        }

        // Match &str to WrapMode
        let str_to_wrapmode = |s: &str| {
            match s {
                "clamp" => WrapMode::Clamp,
                "repeat" => WrapMode::Tile,
                "mirror" => WrapMode::Mirror,
                "border" => WrapMode::Border,
                _ => WrapMode::Clamp,
            }
        };


        // Window dimensions
        let width = matches.value_of("width").unwrap().parse()?;
        let height = matches.value_of("height").unwrap().parse()?;

        // Check to see if they want an example run
        let examplename = matches.value_of("example").map(&str_to_string);

        // Fragment shader path
        let shaderpath = matches.value_of("shader").map(&str_to_string);

        // Texture paths
        let texture0path = matches.value_of("texture0").map(&str_to_string);
        let texture1path = matches.value_of("texture1").map(&str_to_string);
        let texture2path = matches.value_of("texture2").map(&str_to_string);
        let texture3path = matches.value_of("texture3").map(&str_to_string);

        // Texture wrapping
        let wrap0 = matches.value_of("wrap0").map(&str_to_wrapmode);
        let wrap1 = matches.value_of("wrap1").map(&str_to_wrapmode);
        let wrap2 = matches.value_of("wrap2").map(&str_to_wrapmode);
        let wrap3 = matches.value_of("wrap3").map(&str_to_wrapmode);

        // Anistropic filter max value
        let anisotropic_max = matches.value_of("anisotropic_max").map(&str_to_anisotropic_max);

        // Match &str to FilterMethod
        let str_to_filtermethod = |s: &str| {
            match s {
                "scale" => FilterMethod::Scale,
                "mipmap" => FilterMethod::Mipmap,
                "bilinear" => FilterMethod::Bilinear,
                "trilinear" => FilterMethod::Trilinear,
                "anisotropic" => FilterMethod::Anisotropic(anisotropic_max.unwrap()),
                _ => FilterMethod::Bilinear,
            }
        };

        // Texture wrapping
        let filter0 = matches.value_of("filter0").map(&str_to_filtermethod);
        let filter1 = matches.value_of("filter1").map(&str_to_filtermethod);
        let filter2 = matches.value_of("filter2").map(&str_to_filtermethod);
        let filter3 = matches.value_of("filter3").map(&str_to_filtermethod);

        // Window title
        let title = matches.value_of("title").map(&str_to_string);

        // Check to see if they want to download a shader (and then run it)
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

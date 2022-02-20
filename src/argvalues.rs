use error;
use gfx::texture::WrapMode;

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

        // Match &str to WrapMode
        let str_to_wrapmode = |s: &str| {
            match s {
                "clamp" => WrapMode::Clamp,
                "tile" => WrapMode::Tile,
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
            examplename,
            getid,
            andrun,
            title,
        })
    }
}

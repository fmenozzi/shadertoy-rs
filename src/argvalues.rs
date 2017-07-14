use error;

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

    // Some(name) if running an example
    pub examplename: Option<String>,

    // Some(id) if downloading a shader
    pub getid: Option<String>,
}

impl ArgValues {
    pub fn from_cli() -> error::Result<ArgValues> {
        // Load CLI matches
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();

        // Closure for converting &str to String
        let str_to_string = |s: &str| s.to_string();

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

        // Check to see if they want to download a shader
        let getid = if let Some(getmatches) = matches.subcommand_matches("get") {
            getmatches.value_of("id").map(&str_to_string)
        } else {
            None
        };

        Ok(ArgValues {
            width: width,
            height: height,
            shaderpath: shaderpath,
            texture0path: texture0path,
            texture1path: texture1path,
            texture2path: texture2path,
            texture3path: texture3path,
            examplename: examplename,
            getid: getid,
        })
    }
}

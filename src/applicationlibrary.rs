//! Application Library Unison Object (uno) syntax
//!
//!

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;

#[derive(Clone)]
pub struct MyAppsLib {
    name: String,
    path: String,
    build_type: String,
    sources: Vec<String>,
    includes: Vec<String>,
}

impl Default for MyAppsLib {
    fn default() -> Self {
        let mut default_source = Vec::new();
        let mut default_header = Vec::new();

        default_source.push("myAppslib.cpp".to_string());
        default_header.push("myAppslib.h".to_string());

        MyAppsLib {
            name: "myApplicationLibrary".to_string(),
            path: "../Libraries/AppsLib".to_string(),
            build_type: "__Debug".to_string(),
            sources: default_source,
            includes: default_header,
        }
    }
}

pub fn gen_appslib(writer: &mut BufWriter<&File>, params: MyAppsLib) -> Result<(), io::Error> {
    writeln!(writer, "__ApplicationsLibrary {} {{", params.name)?;
    writeln!(writer, "__LibraryPath = \"{}\"; ", params.path)?;
    writeln!(writer, "__LibraryType = \"{}\"; ", params.build_type)?;
    for file in params.sources {
        writeln!(writer, "__Source = \"{}\"; ", file)?;
    }
    for file in params.includes {
        writeln!(writer, "__Include = \"{}\"; ", file)?;
    }
    writeln!(writer, "}}")?;

    Ok(())
}

pub fn call_me() {
    println!("Call Me");
}

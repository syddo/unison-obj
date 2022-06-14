//! Application Library Unison Object (uno) syntax
//!
//!

use std::fs::{File};
use std::io::prelude::*;
use std::io;
use std::io::BufWriter;

//#[derive(Default)]
pub struct MyAppsLib {
    name: String,
    path: String,
    build_type: String,
    sources: Vec<String>,
    includes: Vec<String>,
}

pub fn gen_appslib( writer: &mut BufWriter<File>, params: MyAppsLib) -> Result<(), io::Error> {
    
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
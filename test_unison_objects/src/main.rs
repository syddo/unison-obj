use ::std::fs::File;
use std::io::BufWriter;
use unison_objects;

fn main() {
    println!("Hello, world!");

    let my_appslib_params = unison_objects::MyAppsLib::default();

    let outputfile = File::create("./TestFile.uno").expect("Unable to create output file.");
    let mut writer = BufWriter::new(&outputfile);

    unison_objects::gen_appslib(&mut writer, my_appslib_params).unwrap();
}

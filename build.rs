use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    // actions
    glib_build_tools::compile_resources(
        &["data"],
        "data/nop_name.gresource.xml",
        "nop_name.gresource",
    );

    let current_dir = env::current_dir().unwrap();
    let idir = out_dir.into_string().unwrap();
    let ifile = PathBuf::from(idir).join("nop_name.gresource");
    let odir = current_dir.join("data_store");
    let ofile = odir.join("nop_name.gresource");
    if !odir.exists() {
        let _ = fs::create_dir_all(odir);
    }

    println!("ifile: {:?}", ifile);
    println!("ofile: {:?}", ofile);
    let _ = fs::copy(ifile, ofile);
}

fn main() {
    // actions
    glib_build_tools::compile_resources(
        &["data"],
        "data/nopname.gresource.xml",
        "nopname.gresource",
    );
}

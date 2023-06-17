extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");
    res.set_manifest_file("build.xml");
    res.compile().unwrap();
}

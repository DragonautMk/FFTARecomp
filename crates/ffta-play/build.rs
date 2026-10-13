fn main() {
    #[cfg(windows)] {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../installer/assets/icon.ico");
    res.compile().unwrap();
    }
}

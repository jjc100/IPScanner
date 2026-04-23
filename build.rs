fn main() {
    println!("cargo:rerun-if-changed=assets/ipscanner.ico");

    #[cfg(windows)]
    {
        let mut resource = winres::WindowsResource::new();
        resource.set_icon("assets/ipscanner.ico");
        resource.set("ProductName", "IPScanner");
        resource.set("FileDescription", "IPScanner");
        resource.set("InternalName", "IPScanner");
        resource.set("OriginalFilename", "IPScanner.exe");

        resource
            .compile()
            .expect("failed to compile Windows resources");
    }
}

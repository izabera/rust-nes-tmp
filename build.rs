fn main() {
    cc::Build::new()
        .compiler("clang")
        .target("mos-nes")
        .file("src/rti.s")
        .compile("rti");
}

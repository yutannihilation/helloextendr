fn main() {
    let metadata = helloextendr::get_helloextendr_metadata();
    print!(
        "{}",
        metadata.make_r_wrappers(true, "helloextendr").unwrap()
    );
}

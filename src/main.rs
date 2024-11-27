fn main() {
    println!("Hello, world!");

    let bytes = std::fs::read("test.pdf").unwrap();
    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    println!("{}", out);
    // assert!(out.contains("This is a small demonstration"));
}

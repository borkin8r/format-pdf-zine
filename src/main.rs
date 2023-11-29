use lopdf::Document;
use std::str::FromStr;

fn main() {
    // Collect command line arguments: input_file angle output_file
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 3, "Not enough arguments: input_file output_file");
    let input_file = &args[1];
    let output_file = &args[2];
    let mut doc = Document::load(input_file).unwrap();

    // Note: this example sets Rotate on each page individually for flexibility,
    //  but you can also set it on any node in the page tree and child pages will
    //  inherit the value.
    for (page_number, page_id) in doc.get_pages() {
        if page_number % 2 == 0 {
            let page_dict = doc.get_object_mut(page_id)
            .and_then(|obj| obj.as_dict_mut())
            .expect("Missing page!");

            // Get the current rotation if any; the default is 0
            let current_rotation = page_dict
                .get(b"Rotate")
                .and_then(|obj| obj.as_i64())
                .unwrap_or(0);

            // Add the angle and update
            page_dict.set("Rotate", (current_rotation + 180) % 360); // 180 every even page so that it is mirrored for double sided printing
        }
    }
    // Store file in current working directory.
    doc.save(output_file).unwrap();
}

extern crate graphics_engine; // Use the library's name defined in its `Cargo.toml`

fn main() {
    // Replace with your printer's name or IP (e.g., "192.168.1.100:9100" for a network printer)
    let printer_name = "192.168.1.100:9100";

    // Print Text Example
    let text = "Hello, ESC/POS Printer!";
    let text_result = unsafe {
        graphics_engine::print_text(printer_name.as_ptr() as *const i8, text.as_ptr() as *const i8)
    };

    if text_result == 0 {
        println!("Text printed successfully!");
    } else {
        eprintln!("Failed to print text.");
    }

    // Cut Paper Example
    let cut_result = unsafe {
        graphics_engine::cut_paper(printer_name.as_ptr() as *const i8, true)
    };

    if cut_result == 0 {
        println!("Paper cut successfully!");
    } else {
        eprintln!("Failed to cut paper.");
    }
}
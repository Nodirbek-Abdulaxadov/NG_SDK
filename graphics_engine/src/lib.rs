use std::ffi::{CStr, CString};
use std::io::Write;
use std::net::TcpStream;
use std::os::raw::c_char;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

// ESC/POS Command Constants
const ESC: u8 = 0x1B; // ESC character for commands
const GS: u8 = 0x1D;  // GS character for advanced commands

static mut EVENT_LOOP: Option<EventLoop<()>> = None;
static mut WINDOW: Option<winit::window::Window> = None;

/// Represents a printer object
pub struct Printer {
    printer_name: String,
}

impl Printer {
    /// Creates a new Printer instance with the given name
    pub fn new(printer_name: &str) -> Self {
        Printer {
            printer_name: printer_name.to_string(),
        }
    }

    /// Sends raw data to the printer
    fn send_to_printer(&self, data: &[u8]) -> Result<(), &'static str> {
        match TcpStream::connect(&self.printer_name) {
            Ok(mut stream) => {
                stream.write_all(data).map_err(|_| "Failed to write to printer")?;
                stream.flush().map_err(|_| "Failed to flush printer stream")?;
                Ok(())
            }
            Err(_) => Err("Failed to connect to printer"),
        }
    }

    /// Prints text to the printer
    pub fn print_text(&self, text: &str) -> Result<(), &'static str> {
        let mut data = vec![ESC, b'@']; // Initialize printer with ESC/POS reset command
        data.extend_from_slice(text.as_bytes());
        data.push(b'\n'); // Add newline for each line of text

        self.send_to_printer(&data)
    }

    /// Cuts paper on the printer
    pub fn cut_paper(&self, full_cut: bool) -> Result<(), &'static str> {
        let mut data = vec![GS, b'V', if full_cut { 0x00 } else { 0x01 }]; // Full or partial cut
        self.send_to_printer(&data)
    }
}

#[no_mangle]
pub extern "C" fn initialize_window() -> *const c_char {
    unsafe {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Rust Graphics Window")
            .with_inner_size(LogicalSize::new(800.0, 600.0))
            .build(&event_loop)
            .unwrap();

        EVENT_LOOP = Some(event_loop);
        WINDOW = Some(window);

        CString::new("Window initialized successfully!")
            .unwrap()
            .into_raw()
    }
}

#[no_mangle]
pub extern "C" fn run_event_loop() {
    unsafe {
        if let Some(event_loop) = EVENT_LOOP.as_mut() {
            if let Some(_window) = WINDOW.as_mut() {
                event_loop.run_return(|event, _, control_flow| {
                    *control_flow = ControlFlow::Poll;

                    match event {
                        Event::WindowEvent { event, .. } => match event {
                            WindowEvent::CloseRequested => {
                                *control_flow = ControlFlow::Exit;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                });
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn terminate_window() {
    unsafe {
        EVENT_LOOP = None;
        WINDOW = None;
    }
}

/// Prints the given text to the ESC/POS printer.
#[no_mangle]
pub extern "C" fn print_text(printer_name: *const c_char, text: *const c_char) -> i32 {
    let printer_name = unsafe { CStr::from_ptr(printer_name) }.to_string_lossy().to_string();
    let text = unsafe { CStr::from_ptr(text) }.to_string_lossy().to_string();

    let printer = Printer::new(&printer_name);

    match printer.print_text(&text) {
        Ok(_) => 0, // Success
        Err(_) => -1, // Failure
    }
}

/// Cuts the paper on the ESC/POS printer.
#[no_mangle]
pub extern "C" fn cut_paper(printer_name: *const c_char, full_cut: bool) -> i32 {
    let printer_name = unsafe { CStr::from_ptr(printer_name) }.to_string_lossy().to_string();

    let printer = Printer::new(&printer_name);

    match printer.cut_paper(full_cut) {
        Ok(_) => 0, // Success
        Err(_) => -1, // Failure
    }
}

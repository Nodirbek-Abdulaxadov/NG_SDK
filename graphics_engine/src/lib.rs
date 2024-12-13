use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

static mut EVENT_LOOP: Option<EventLoop<()>> = None;
static mut WINDOW: Option<winit::window::Window> = None;

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
            if let Some(window) = WINDOW.as_mut() {
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
extern crate winit;
extern crate winapi;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::{WindowBuilderExtWindows, WindowExtWindows},
    window::WindowBuilder,
};

use winapi::um::winuser::{BeginPaint, EndPaint, ReleaseCapture, SendMessageW, PAINTSTRUCT, WM_NCLBUTTONDOWN, WM_PAINT, FillRect};
use winapi::um::wingdi::{CreateSolidBrush, RGB};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("click 'n' drag")
        .with_decorations(false)
        .with_inner_size(winit::dpi::LogicalSize::new(100, 100))
        .build(&event_loop)
        .unwrap();

    window.set_outer_position(winit::dpi::PhysicalPosition::new(500, 500));
    

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                unsafe {
                    let hwnd = window.hwnd() as *mut _;
                    let mut ps: PAINTSTRUCT = std::mem::zeroed();
                    let hdc = BeginPaint(hwnd, &mut ps);
                    let brush = CreateSolidBrush(RGB(21, 51, 153));
                    FillRect(hdc, &ps.rcPaint, brush);
                    EndPaint(hwnd, &ps);
                }
            }
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::MouseInput { state, .. } => {
                    if state == winit::event::ElementState::Pressed {
                        unsafe {
                            let hwnd = window.hwnd() as *mut _;
                            ReleaseCapture();
                            SendMessageW(hwnd, WM_NCLBUTTONDOWN, 2, 0); // 2 is HTCAPTION
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    });
}

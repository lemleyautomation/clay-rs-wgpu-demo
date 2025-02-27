use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::graphics::graphics_context::GraphicsContext;

#[derive(Default)]
pub struct App<'a> {
    ctx: Option<GraphicsContext<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Clay-rs-WGPU-Demo".to_string())
            .with_inner_size(LogicalSize::new(800, 600));

        let window = event_loop.create_window(window_attributes).unwrap();

        let dpi_scale = window.scale_factor() as f32;

        let state = GraphicsContext::new(window);

        state.ui_state.borrow_mut().dpi_scale = dpi_scale;

        self.ctx = Some(state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Resized(_) => {
                self.ctx.as_mut().unwrap().resize();
            }
            WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer:_ } => {
                self.ctx.as_mut().unwrap().ui_state.borrow_mut().dpi_scale = scale_factor as f32;
            }
            WindowEvent::RedrawRequested => {
                self.ctx.as_mut().unwrap().render().unwrap();
                self.ctx.as_mut().unwrap().clay_user_data.mouse_down_rising_edge = false;
                //std::thread::sleep(Duration::from_millis(16));
                self.ctx.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::MouseInput { device_id:_, state, button } => {
                match button {
                    winit::event::MouseButton::Left => {
                        self.ctx.as_mut().unwrap().clay_user_data.mouse_down_rising_edge = state.is_pressed();
                    }
                    _ => {}
                }
            }
            WindowEvent::MouseWheel { device_id:_, delta, phase:_ } => {
                self.ctx.as_mut().unwrap().clay_user_data.scroll_delta = match delta {
                    MouseScrollDelta::LineDelta(x,y ) => (x,y),
                    MouseScrollDelta::PixelDelta(position) => position.into()
                };
            }
            WindowEvent::CursorMoved { device_id:_, position } => {
                self.ctx.as_mut().unwrap().clay_user_data.mouse_position = position.into();
            }
            _ => (),
        }
    }
}
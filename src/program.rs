use conrod_core;
use conrod_winit;
use glium::{self, glutin, Surface};
use std;

pub struct Program {
    pub ui: conrod_core::Ui,
    pub display: GliumDisplayWinitWrapper,
    event_loop: EventLoop,
    glium_events_loop: glutin::EventsLoop,
    renderer: conrod_glium::Renderer,
}

enum Continuation {
    Stop,
    Continue,
}

impl Program {
    pub fn new(title: &str, width: u32, height: u32, refresh_time: std::time::Duration) -> Program {
        let glium_events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions((width, height).into());
        let context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &glium_events_loop).unwrap();
        let display = GliumDisplayWinitWrapper(display);
        Program {
            ui: conrod_core::UiBuilder::new([width as f64, height as f64]).build(),
            event_loop: EventLoop::new(refresh_time),
            glium_events_loop: glium_events_loop,
            renderer: conrod_glium::Renderer::new(&display.0).unwrap(),
            display: display,
        }
    }

    fn draw<F>(&mut self, f: &mut F) -> ()
    where
        F: FnMut(&mut conrod_core::UiCell) -> (),
    {
        // Process higher level events (DoubleClick ...) created by Ui::handle_event.
        let ui_cell = &mut self.ui.set_widgets();
        f(ui_cell)
    }

    fn render<Img>(&mut self, image_map: &conrod_core::image::Map<Img>) -> ()
    where
        Img: std::ops::Deref + conrod_glium::TextureDimensions,
        for<'a> glium::uniforms::Sampler<'a, Img>: glium::uniforms::AsUniformValue,
    {
        if let Some(primitives) = self.ui.draw_if_changed() {
            self.renderer.fill(&self.display.0, primitives, image_map);
            let mut target = self.display.0.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0); // needs the Surface trait
            self.renderer
                .draw(&self.display.0, &mut target, image_map)
                .unwrap();
            target.finish().unwrap();
        }
    }

    fn process_events(&mut self) -> Continuation {
        for event in self.event_loop.next(&mut self.glium_events_loop) {
            // Use the `winit` backend to convert the winit event to a conrod one.
            if let Some(ev) = conrod_winit::convert_event(event.clone(), &self.display) {
                self.ui.handle_event(ev);
                self.event_loop.ui_needs_update = true;
            };

            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => return Continuation::Stop,
                    _ => (),
                },
                _ => (),
            };
        }
        Continuation::Continue
    }

    pub fn run<Img, F>(&mut self, image_map: &conrod_core::image::Map<Img>, f: &mut F) -> ()
    where
        Img: std::ops::Deref + conrod_glium::TextureDimensions,
        for<'a> glium::uniforms::Sampler<'a, Img>: glium::uniforms::AsUniformValue,
        F: FnMut(&mut conrod_core::UiCell) -> (),
    {
        'main: loop {
            // Handle all events.
            if let Continuation::Stop = self.process_events() {
                break 'main;
            }

            // Instantiate the widgets.
            self.draw(f);

            // Render the ui and then display it on the screen.
            self.render(image_map);
        }
    }
}

pub struct GliumDisplayWinitWrapper(pub glium::Display);

impl conrod_winit::WinitWindow for GliumDisplayWinitWrapper {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        self.0.gl_window().get_inner_size().map(Into::into)
    }
    fn hidpi_factor(&self) -> f32 {
        self.0.gl_window().get_hidpi_factor() as _
    }
}

struct EventLoop {
    time_step: std::time::Duration,
    last_update: std::time::Instant,
    ui_needs_update: bool,
}

impl EventLoop {
    fn new(time_step: std::time::Duration) -> Self {
        EventLoop {
            time_step,
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    fn next(&mut self, events_loop: &mut glutin::EventsLoop) -> Vec<glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let duration_since_last_update = std::time::Instant::now().duration_since(self.last_update);
        if duration_since_last_update < self.time_step {
            std::thread::sleep(self.time_step - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }
}

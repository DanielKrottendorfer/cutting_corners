use glium::Surface;

#[path = "../game_s.rs"]
pub mod my_game_logic;


pub struct CCGameEngine{
    pub event_loop: glutin::EventsLoop,
    display: glium::Display,
    running: bool,
    game_logic: my_game_logic::CCGame,
    renderer: my_game_logic::my_renderer::Renderer,
}

impl CCGameEngine {

    pub fn new(game: my_game_logic::CCGame,title: &str, vsync:bool) ->CCGameEngine{
        use glium::glutin;

        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new().with_title(title);
        let context = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(vsync);
        let mut display = glium::Display::new(window, context, &events_loop).unwrap();

        let renderer = my_game_logic::my_renderer::Renderer::new(&mut display);


        CCGameEngine{
            event_loop: (events_loop),
            display: (display),
            running: true,
            game_logic: game,
            renderer: (renderer),
        }
    }

    fn init(&mut self){
        self.game_logic.init(&mut self.display);
        self.display.gl_window().window().grab_cursor(true).unwrap();
        self.display.gl_window().window().hide_cursor(true);
    }

    fn input(&mut self){
        self.game_logic.input(&mut self.event_loop);

    }

    fn update(&mut self, dt: &f32){
        if !self.game_logic.running {
            self.running = false;
        }
        self.game_logic.update(&dt);

        if self.game_logic.mode_changed{
            if self.game_logic.ego_mode {
                self.display.gl_window().window().grab_cursor(true).unwrap();
                self.display.gl_window().window().hide_cursor(true);
            }else {
                self.display.gl_window().window().grab_cursor(false).unwrap();
                self.display.gl_window().window().hide_cursor(false);
            }
            self.game_logic.mode_changed = false;
        }

    }

    fn render(&mut self){
        let mut target_frame = self.display.draw();
        target_frame.clear_color_and_depth((1.0, 1.0, 0.0, 1.0), 1.0);
        self.game_logic.render(&mut self.renderer,&mut target_frame,&mut self.display);
        target_frame.finish().unwrap();
    }

    pub fn start(&mut self){

        self.init();

        use time::PreciseTime;

        let mut previous = PreciseTime::now();
        let mut lag: i64 = 0;
        let mcs_per_update: i64 = 10000;
        let mut fpsc = 0;
        let mut start = PreciseTime::now();

        while self.running {
            self.input();

            let current = PreciseTime::now();
            let elapsed: i64;
            match previous.to(current).num_microseconds() {
                Some(x) => elapsed = x,
                None => elapsed = std::i64::MAX,
            }
            previous = current;
            lag += elapsed;

            while lag >= mcs_per_update {
                self.update(&(mcs_per_update as f32 / 1000.0));
                lag -= mcs_per_update;
            }

            let end = PreciseTime::now();
            fpsc+=1;
            if start.to(end).num_seconds()>=1{
                println!("fps: {}",fpsc);
                fpsc=0;
                start = PreciseTime::now();
            }
            if self.game_logic.ready_in_que.load(std::sync::atomic::Ordering::SeqCst) > 0  {
                self.game_logic.load_que(&mut self.display);
                self.game_logic.ready_in_que.fetch_sub(1,std::sync::atomic::Ordering::SeqCst);
            }
            self.render();
        }
    }
}


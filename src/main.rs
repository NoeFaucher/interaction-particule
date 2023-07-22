
    // let test_or = Particule::new_origin(Color::Green, 1.0);
    // let test_rd = Particule::new_rand(100.0,100.0, Color::Blue, 1.0);

    // let mut tests = vec![test_or,test_rd];


    // println!("av test_or : {:?}",tests[0]);
    // println!("av test_rd : {:?}",tests[1]);
    
    // for j in 0..20 {
    //     Particule::update(&mut tests);

    //     println!("ap {} test_or : {:?}",j,tests[0]);
    //     println!("ap {} test_rd : {:?}",j,tests[1]);



use std::f64::consts::PI;

use gtk::gdk::FrameClock;
use rand::Rng;

use gtk::cairo::Context;
use gtk::{self, glib};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea, Box, Orientation};



fn draw_func(area: &DrawingArea, cr : &Context, width: i32, height: i32) {
    
    let context = area.style_context();

    // cr.arc(1.0,1.0,30.0,0.0,2.0*PI) ;
    let mut rng = rand::thread_rng();
    let radius = rng.gen_range(20.0..70.0);

    cr.arc(width as f64 / 2.0, height as f64 / 2.0, radius, 0.0, 2.0*PI);

    let color = context.color();
    cr.set_source_rgba(color.red() as f64, color.green() as f64, color.blue() as f64, color.alpha() as f64);

    let _ = cr.fill();
    
}

fn time_handler(area: &DrawingArea, fc: &FrameClock) -> Continue {

    area.queue_draw();

    return Continue(true);
}


fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("First GTK Program")
        .default_width(500)
        .default_height(500)
        .build();

    let container_box = Box::new(Orientation::Horizontal,10);

    let drawing_area = DrawingArea::new();

    drawing_area.set_content_width(300);
    drawing_area.set_content_height(300);

    drawing_area.set_draw_func(draw_func);

    drawing_area.add_tick_callback(time_handler);

    container_box.append(&drawing_area);

    window.set_child(Some(&container_box));
    


    window.show();

}





fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();


    application.connect_activate(build_ui);

    application.run()
}
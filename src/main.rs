

use rand::Rng;
use std::collections::HashMap;

use gtk::gdk::FrameClock;

use gtk::cairo::Context;
use gtk::{self, glib};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea, Box, Orientation};

mod particule;
use particule::{Particule, Vect, update};

mod config_constant;
use config_constant::{WIDTH, HEIGHT, NB_RED, NB_BLUE, NB_GREEN, NB_YELLOW};

/** drawing function which draw on the drawing area
 * 
 */
fn draw_func(area: &DrawingArea, cr : &Context, width: i32, height: i32, particules_map: &mut HashMap<String,Vec<Particule>>) {
    
    let fc = area.frame_clock().unwrap();

    cr.rectangle(0., 0., width as f64, height as f64);
    cr.set_source_rgb(0., 0., 0.);
    cr.fill().unwrap();

    for (_,particules) in particules_map.iter() {
        for particule in particules.iter() {
            particule.draw(&cr);
        }
    }

    update(particules_map);



    
}

/** Function called each frame
 * 
 */
fn time_handler(area: &DrawingArea, fc: &FrameClock) -> Continue {

    println!("{}",fc.fps());

    // redraw the drawing area by calling the drawing area again
    area.queue_draw();

    return Continue(true);
}




fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("First GTK Program")
        .default_width(WIDTH as i32)
        .default_height(HEIGHT as i32)
        .build();

    let container_box = Box::new(Orientation::Horizontal,10);


    let drawing_area = DrawingArea::new();

    drawing_area.set_content_width(WIDTH as i32);
    drawing_area.set_content_height(HEIGHT as i32);


    // content data
    let mut particules: HashMap<String,Vec<Particule>> = HashMap::new();



    let mut blues: Vec<Particule> = vec![];
    for _ in 0..NB_BLUE{
        blues.push(Particule::new(
            Vect::new_rand(WIDTH, HEIGHT),
            Vect::new_zero(),
            Vect::new_zero(),
            particule::Color::Blue(0.,0.,1.),
            rand::thread_rng().gen::<f64>() * 2.,
        ));
    }
    particules.insert(String::from("blue"), blues);

    let mut reds: Vec<Particule> = vec![];
    for _ in 0..NB_RED{
        reds.push(Particule::new(
            Vect::new_rand(WIDTH, HEIGHT),
            Vect::new_zero(),
            Vect::new_zero(),
            particule::Color::Red(1.,0.,0.),
            rand::thread_rng().gen::<f64>() * 2.,
        ));
    }
    particules.insert(String::from("red"), reds);

    let mut greens: Vec<Particule> = vec![];
    for _ in 0..NB_GREEN{
        greens.push(Particule::new(
            Vect::new_rand(WIDTH, HEIGHT),
            Vect::new_zero(),
            Vect::new_zero(),
            particule::Color::Red(0.,1.,0.),
            rand::thread_rng().gen::<f64>() * 2.,
        ));
    }
    particules.insert(String::from("green"), greens);

    let mut yellows: Vec<Particule> = vec![];
    for _ in 0..NB_YELLOW{
        yellows.push(Particule::new(
            Vect::new_rand(WIDTH, HEIGHT),
            Vect::new_zero(),
            Vect::new_zero(),
            particule::Color::Yellow(1.,1.,0.),
            rand::thread_rng().gen::<f64>() * 2.,
        ));
    }

    particules.insert(String::from("yellow"), yellows);


    // set the drawing function
    drawing_area.set_draw_func( move |area: &DrawingArea, cr : &Context, width: i32, height: i32| {
        draw_func(area, cr, width, height, &mut particules)
    });

    // add the function called each frame
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
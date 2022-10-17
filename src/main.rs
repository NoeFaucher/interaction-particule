use std::io;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow,DrawingArea,Button};
use cairo_rs as cairo;
use cairo::{Context,ImageSurface,Format};
use std::f64::consts::PI;


mod particule;
use crate::particule::{Particule,Color};



mod config_constant;

fn main() {
    
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(350)
            .build();
        
        let da = DrawingArea::new();

        let surface = ImageSurface::create(Format::ARgb32,350,350).unwrap();
        let cr = Context::new(&surface).unwrap();

        // context.save().expect("unable to save");
        // //context.translate(0_f64, 0_f64);
        // context.scale(350_f64 / 2.0, 350_f64 / 2.0);
        // context.arc(0.0, 0.0, 1.0, 0.0, 2_f64 * PI);
        // context.restore().expect("unable to restore");
        // context.save();
        // context.arc(1.0, 1.0, 30.0, 0.0, 2.0 * PI); // full circle
        // context.set_source_rgb(255.0, 255.0, 255.0);    // partially translucent
        // context.fill_preserve();
        // context.restore();  // back to opaque black
        // context.stroke();

        // context.save();

        // cr.set_source_rgb(0.0, 0.0, 0.0);
        // cr.set_line_width(10.0);

        // cr.move_to(0.0, 0.0);
        // cr.line_to(300.0, 300.0);

        // cr.stroke().expect("fail to stroke");

        let f =150;

        let width = surface.width() as f64;
        let height = surface.height() as f64;
        let dx = f as f64 * 1.0;

        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.paint();

        let cx = width / 2.0;
        let cy = height / 2.0;
        let r = cy;
        let cstart = -0.5 * PI;
        let cend = cstart + 2.0 * PI * ((f + 1) as f64) / 300.0;
        cr.move_to(cx, cy);
        cr.line_to(cx, 0.0);
        cr.arc(cx, cy, r, cstart, cend);
        cr.line_to(cx, cy);
        cr.set_source_rgba(0.0, 0.5, 0.0, 0.2);
        cr.fill();

        cr.select_font_face(
            "sans-serif",
            cairo::FontSlant::Normal,
            cairo::FontWeight::Normal,
        );
        cr.set_font_size(70.0);
        cr.move_to(600.0 - dx, 100.0);
        cr.set_source_rgb(0.0, 0.0, 1.0);
        cr.show_text("Hello, world! 1234567890");
        cr.fill();

        da.draw(&cr);
       
        window.add(&da);

        window.show_all();
    });

    application.run();


    // let test_or = Particule::new_origin(Color::Green, 1.0);
    // let test_rd = Particule::new_rand(100.0,100.0, Color::Blue, 1.0);

    // let mut tests = vec![test_or,test_rd];


    // println!("av test_or : {:?}",tests[0]);
    // println!("av test_rd : {:?}",tests[1]);
    
    // for j in 0..20 {
    //     Particule::update(&mut tests);

    //     println!("ap {} test_or : {:?}",j,tests[0]);
    //     println!("ap {} test_rd : {:?}",j,tests[1]);
    // }
}

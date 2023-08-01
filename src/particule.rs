use rand::Rng;
use gtk::cairo::Context;
use std::{f64::consts::PI, borrow::BorrowMut, collections::HashMap};

use num::Num;

use crate::config_constant::{WIDTH, HEIGHT};

// affichage graph GTK - cairo

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Green(f64,f64,f64),
    Yellow(f64,f64,f64),
    Blue(f64,f64,f64),
    Red(f64,f64,f64) 
}


#[derive(Debug, Clone, Copy)]
pub struct Particule {
    pos: Vect<f64>,
	vel: Vect<f64>,
	acc: Vect<f64>,
	color: Color,
    mass: f64,
}



impl Particule {

    pub fn new(_pos: Vect<f64>, _vel : Vect<f64>, _acc: Vect<f64>, _color: Color, _mass: f64) -> Particule {
        Particule {
            pos: _pos,
            vel: _vel,
            acc: _acc,
            color: _color,
            mass: _mass
        }
    }



    pub fn draw(&self,  cr : &Context) {
        cr.arc(self.pos.x, self.pos.y, 2.5 * self.mass + 5., 0.0, 2.0 * PI );

        let (r,g,b) = match self.color {
            Color::Blue(r,g,b) => (r,g,b),
            Color::Green(r,g,b) => (r,g,b),
            Color::Red(r,g,b) => (r,g,b),
            Color::Yellow(r,g,b) => (r,g,b),
        };

        cr.set_source_rgb(r, g, b); // Blue color
        cr.fill().unwrap();
    }
	


}


    pub fn update(particules_map: &mut HashMap<String,Vec<Particule>>) {

        // let test = particules_map.clone();

        // interaction(particules, &test , 1.0);
        

        rule(String::from("red"),String::from("red"),-1.,particules_map);
        rule(String::from("red"),String::from("blue"),-1.,particules_map);
        rule(String::from("red"),String::from("green"),-0.5,particules_map);
        rule(String::from("red"),String::from("yellow"),-0.5,particules_map);
        // rule(String::from("blue"),String::from("red"),-1.,particules_map);

        rule(String::from("blue"),String::from("blue"),0.05,particules_map);
        rule(String::from("blue"),String::from("red"),-1.,particules_map);
        rule(String::from("blue"),String::from("green"),-1.,particules_map);
        rule(String::from("blue"),String::from("yellow"),-1.,particules_map);

        rule(String::from("green"),String::from("blue"),-1.,particules_map);
        rule(String::from("green"),String::from("red"),-1.,particules_map);
        rule(String::from("green"),String::from("green"),-1.,particules_map);
        rule(String::from("green"),String::from("yellow"),-2.,particules_map);

        rule(String::from("yellow"),String::from("blue"),-1.,particules_map);
        rule(String::from("yellow"),String::from("red"),-0.5,particules_map);
        rule(String::from("yellow"),String::from("green"),0.1,particules_map);
        rule(String::from("yellow"),String::from("yellow"),-5.,particules_map);
    }

pub fn rule(group1_name: String, group2_name: String, g: f64, particules_map: &mut HashMap<String,Vec<Particule>>)  {
    let particules_map_copy = particules_map.clone();
    
    interaction(particules_map.get_mut(&group1_name).unwrap(), particules_map_copy.get(&group2_name).unwrap(), g)

    // return  particules_map.get(&group1_name).unwrap().clone();
}


pub fn interaction(
    group1: &mut [Particule],
    group2: &[Particule],
    g: f64,
) {
    let g = -g;


    for (i,a) in group1.borrow_mut().iter_mut().enumerate() {

        for (j,b) in group2.into_iter().enumerate() {

            if (i ==j) {
                continue;
            }
            a.acc.add(a.pos.clone().sub(b.pos).normalize().mult(g*b.mass* (1. / a.pos.dist(b.pos)) * (1. / a.pos.dist(b.pos))));

            

            // let dx = a.pos.dist_x(b.pos);
            // let dy = a.pos.dist_y(b.pos);



        //     let mut d =  (dx*dx + dy*dy).sqrt();

        //     if d == 0.0 {
        //         if i == j {
        //             continue;
        //         } else {
        //             d = 0.01;
        //         }
        //     }
            
        //     let f = g * b.mass / d;

        //     new_acc.add(Vect::new(
        //         f * dx,
        //         f * dy
        //     ));

        }


        a.vel.add(a.acc);
        a.pos.add(a.vel.mult(0.999));
        a.acc = Vect::new_zero();
        
        if(a.pos.x <= 0.0 || a.pos.x >= WIDTH) {
            a.vel.x = a.vel.x * (-1.0);

            if (a.pos.x >= WIDTH) {
                a.pos.x = WIDTH;
            }
            if (a.pos.x <= 0.0) {
                a.pos.x = 0.0;
            }

        }
        if(a.pos.y <= 0.0 || a.pos.y >= HEIGHT) {
            a.vel.y = a.vel.y * (-1.0);
            if (a.pos.y >= HEIGHT) {
                a.pos.y = HEIGHT;
            }
            if (a.pos.y <= 0.0) {
                a.pos.y = 0.0;
            }
        }



    }



}




#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vect<T> 
where
    T: Num,
{
    x: T,
    y: T,
}

impl Vect<f64> 
{
    pub fn new(_x: f64, _y:f64) -> Vect<f64> {
        Vect {
            x : _x,
            y : _y,
        }
    }

    pub fn new_zero() -> Vect<f64> {
        Self::new(0.0,0.0)
    }

    pub fn new_rand(max_x: f64, max_y: f64) -> Vect<f64> {
        Vect {
            x : rand::thread_rng().gen::<f64>() * max_x,
            y : rand::thread_rng().gen::<f64>() * max_y
        }
    }


    pub fn add(&mut self, other: Self) -> Self{
        self.x += other.x;
        self.y += other.y;
        return *self;
    } 

    pub fn sub(&mut self, other: Self) -> Self{
        self.x -= other.x;
        self.y -= other.y;
        return *self;
    }

    pub fn mult(&mut self, val: f64) -> Self {
        self.x *= val;
        self.y *= val;
        return *self;
    }

    pub fn normalize(&mut self) -> Self {
        self.mult(1. / self.dist(Vect::new_zero()))
    }

    pub fn dist(&self, b: Self) -> f64 {
        ((self.x- b.x).powi(2) + (self.y - b.y).powi(2) ).sqrt()
    }

    pub fn dist_x(&self, b: Self) -> f64 {
        self.x- b.x
    }
    
    pub fn dist_y(&self, b: Self) -> f64 {
        self.y- b.y
    }

}


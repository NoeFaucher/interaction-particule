use rand::Rng;

// affichage graph GTK - cairo

#[derive(Debug, PartialEq)]
pub enum Color {
    Green,
    Yellow,
    Blue,
    Red 
}

#[derive(Debug, PartialEq)]
struct Vect {
    x: f64,
    y: f64,
}

#[derive(Debug)]
pub struct Particule {
    pos: Vect,
	vel: Vect,
	acc: Vect,
	color: Color,
    mass: f64,
}

impl Vect {
	fn new_origin() -> Vect {
		Vect {
			x : 0.0,
			y : 0.0
		}
	}

    fn new_rand(max_x: f64, max_y: f64) -> Vect {
        Vect {
            x : rand::thread_rng().gen::<f64>() * max_x,
            y : rand::thread_rng().gen::<f64>() * max_y
        }
    }

    fn update(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    } 

    fn dist_x(&self, b: &Self) -> f64 {
        b.x - self.x
    }
    
    fn dist_y(&self, b: &Self) -> f64 {
        b.y - self.y
    }
}


impl Particule {
    pub fn new_origin(color: Color, mass: f64) -> Particule {
        Particule {
            pos : Vect::new_origin(),
            vel : Vect::new_origin(),
            acc : Vect::new_origin(),
            color : color,
            mass : mass
        }
    }

    pub fn new_rand(max_x: f64, max_y: f64, color: Color, mass: f64) -> Particule {
        Particule {
            pos : Vect::new_rand(max_x,max_y),
            vel : Vect::new_rand(max_x,max_y),
            acc : Vect::new_rand(max_x,max_y),
            color : color,
            mass : mass
        }
    }

    pub fn update(particules: &mut [Particule]) {
        for i in 0..particules.len() {
            Self::update_acc(particules, i);
            Self::update_pos_vel(&mut particules[i]);
        }
    }
	
	pub fn update_pos_vel(&mut self) {
        self.pos.update(&self.vel); 
        self.vel.update(&self.acc); 
	}

    pub fn update_acc(particules: &mut [Particule], idx: usize) {
        for i in 0..particules.len() {
            if i == idx { continue; }
            
            let d_x = particules[idx].pos.dist_x(&particules[i].pos);
            let f_x = crate::config_constant::C_TEST * particules[idx].mass * particules[i].mass / (d_x*d_x);
            
            let d_y = particules[idx].pos.dist_y(&particules[i].pos);
            let f_y = crate::config_constant::C_TEST * particules[idx].mass * particules[i].mass / (d_y*d_y);

            particules[idx].acc.x += if d_x > 0.0 {f_x} else{-f_x};
            particules[idx].acc.y += if d_y > 0.0 {f_y} else{-f_y};
            
        }
    }
}


use console_engine::{*, pixel::Pixel};
use term_size::*;
use rand::{thread_rng, Rng};

fn gen_pixel() -> Pixel {
    let mut rng = thread_rng();
    Pixel { 
        bg: Color::Black, 
        fg: Color::Rgb { 
            r: rng.gen(), 
            g: rng.gen(), 
            b: rng.gen() 
        }, 
        chr: '█' 
    }
}

fn change_pixel(pixel: &mut Pixel) {
    match pixel.fg {
        Color::Rgb {r, g, b} => {
            let mut rng = thread_rng();
            if rng.gen_bool(0.1) {
                pixel.fg = gen_pixel().fg;   
            }
            else {
                let mut new_r = if rng.gen_bool(0.5) {
                    r.checked_sub(rng.gen_range(0..=10))
                } else {
                    r.checked_add(rng.gen_range(0..=10)) 
                };
                if new_r.is_none() {
                    new_r = Some(rng.gen::<u8>());
                }
                let mut new_g = if rng.gen_bool(0.5) {
                    g.checked_sub(rng.gen_range(0..=10))
                } else {
                    g.checked_add(rng.gen_range(0..=10)) 
                };
                if new_g.is_none() {
                    new_g = Some(rng.gen::<u8>());
                }
                let mut new_b = if rng.gen_bool(0.5) {
                    b.checked_sub(rng.gen_range(0..=10))
                } else {
                    b.checked_add(rng.gen_range(0..=10)) 
                };
                if new_b.is_none() {
                    new_b = Some(rng.gen::<u8>());
                }
                pixel.fg = Color::Rgb { r: new_r.expect("should not panic"), g: new_g.expect("should not panic"), b: new_b.expect("should not panic") }; 
            }
        }
        _ => panic!("something went wrong.")
    }
}


fn main() {

}

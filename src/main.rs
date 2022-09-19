use macroquad::prelude::*;

struct Block {
    w: f32,
    x: f32,
    v: f64,
    m: f32,
    is_visible: bool
}

impl Block {
    fn new(w: f32, x: f32, v: f64, m: f32) -> Block {
        Block { w, x , v, m, is_visible: true}
    }

    fn draw(&self) {
        draw_rectangle(self.x, screen_height() - (self.w + 50.0), self.w, self.w, BLACK);
    }

    fn update(&mut self) {
        self.x += self.v as f32
    }

    fn collide(&self, other: &Block) -> f64 {
        let sm = self.m as f64;
        let om = other.m as f64;
        ((sm - om) / (sm + om))  * self.v + ((2.0 * om) / (sm + om)) * other.v
    }

    fn is_colliding(&self, other: &Block) -> bool {
        if self.is_visible {
            self.x < other.x + other.w &&
            self.x + self.w > other.x 
        } else {
            false
        }
    }
}

#[macroquad::main("pi_count")]
async fn main() {
    let digit = 6.0;
    let time_steps = 6600;
    let mut block1 = Block::new(30.0, 90.0, 0.0, 1.0);
    let mut block2 = Block::new(60.0, 600.0 , -0.5_f64/time_steps as f64, block1.m * 100.0_f32.powf(digit));
    let mut count = 0;
    loop {
        clear_background(WHITE);
        draw_text(count.to_string().as_str(), 20.0, 20.0, 30.0, BLACK);
        for _ in 0..time_steps {

            if block1.is_colliding(&block2){
                let v1 = block1.collide(&block2);
                let v2 = block2.collide(&block1);
                block1.v = v1;
                block2.v = v2;
                count += 1;
                // println!("block")
            }

            if block1.x <= 0.0 {
                block1.v *= -1.0;
                count += 1;
                // println!("wall")
            }

            block1.update();
            block2.update();
        }
        block1.draw();
        block2.draw();

        if block1.x >= screen_width() {
            block1.is_visible = false
        }
        if block2.x >= screen_width() {
            block2.is_visible = false
        }

        next_frame().await
    }
}

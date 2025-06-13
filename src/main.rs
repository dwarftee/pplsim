use macroquad::prelude::*;

#[macroquad::main("people simulation")]
async fn main() {
   loop {
     clear_background(BLACK);

     next_frame().await;
   }
}

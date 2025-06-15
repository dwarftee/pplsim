use macroquad::prelude::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "people simulation".to_owned(),
        window_width: 800,
        window_height: 700,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    macroquad::rand::srand(macroquad::miniquad::date::now() as u64);
 
    //init images

    let b1 = load_texture("assets/bazinga.png").await.unwrap();
    b1.set_filter(FilterMode::Nearest);

    let b2 = load_texture("assets/balinga.png").await.unwrap();
    b2.set_filter(FilterMode::Nearest);

    let apple = load_texture("assets/apple.png").await.unwrap();
    apple.set_filter(FilterMode::Nearest);

    let water = load_texture("assets/water.png").await.unwrap();
    water.set_filter(FilterMode::Nearest);

    //variables
    
    let red = 59.0/255.0;
    let green = 196.0/255.0; //background colour
    let blue = 127.0/255.0;
    
    let b1x = 100.0; //values of the peoples coordinates
    let b1y = 100.0;
    let b2x = 500.0;
    let b2y = 500.0;

    let apple1x: f32 = macroquad::rand::gen_range(10.0,740.0); //generates random positions for apples 
    let apple1y: f32 = macroquad::rand::gen_range(10.0,640.0); //(mut means that the variable can be changed later)
    let apple2x: f32 = macroquad::rand::gen_range(10.0,740.0);
    let apple2y: f32 = macroquad::rand::gen_range(10.0,640.0);
    let apple3x: f32 = macroquad::rand::gen_range(10.0,740.0);
    let apple3y: f32 = macroquad::rand::gen_range(10.0,640.0);

    let waterx: f32 = macroquad::rand::gen_range(10.0, 740.0);
    let watery: f32 = macroquad::rand::gen_range(10.0, 640.0);

    //main loop

    loop {
        clear_background(Color::new(red,green,blue,1.0));

        draw_texture_ex(
            &b1,
            b1x,
            b1y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(100.0,100.0)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &b2,
            b2x,
            b2y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &water,
            waterx,
            watery,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(200.0, 200.0)),
                ..Default::default()
            },
        );

        draw_texture(&apple,apple1x,apple1y,WHITE);
        draw_texture(&apple,apple2x,apple2y,WHITE);
        draw_texture(&apple,apple3x,apple3y,WHITE);

        next_frame().await;
    }
}

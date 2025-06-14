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
 
    //init images

    let b1 = load_texture("assets/bazinga.png").await.unwrap();
    b1.set_filter(FilterMode::Nearest);

    let b2 = load_texture("assets/balinga.png").await.unwrap();
    b2.set_filter(FilterMode::Nearest);

    //variables

    let red = 59.0/255.0;
    let green = 196.0/255.0;
    let blue = 127.0/255.0;
    
    let b1x = 100.0;
    let b1y = 100.0;
    let b2x = 500.0;
    let b2y = 500.0;
    
    //main loop

    loop {
        clear_background(Color::new(red,green,blue,1.0));

        draw_texture_ex(
            &b1,
            b1x,
            b1y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(200.0,200.0)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &b2,
            b2x,
            b2y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(200.0, 200.0)),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}

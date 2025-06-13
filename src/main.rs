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
    
    let tst1 = load_texture("assets/bazinga.png").await.unwrap();
    tst1.set_filter(FilterMode::Nearest);

    let x = 100.0;
    let y = 100.0;

    loop {
        clear_background(SKYBLUE);

        draw_texture_ex(
            &tst1,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(200.0,200.0)),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}

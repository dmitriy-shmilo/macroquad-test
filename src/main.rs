use std::{
    f32::consts::PI,
    time::{SystemTime, UNIX_EPOCH},
};

use macroquad::{
    prelude::*,
    rand::{gen_range, rand, srand},
};

fn tuple2vec(t: (f32, f32)) -> Vec2 {
    Vec2::new(t.0, t.1)
}

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;
const TOLERANCE: f32 = 3.0;

#[macroquad::main("Hello?")]
async fn main() {
    srand(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("???")
            .as_secs(),
    );

    let render_target = render_target(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Linear);

    let mut render_target_camera =
        Camera2D::from_display_rect(Rect::new(0., 0., SCREEN_WIDTH, SCREEN_HEIGHT));
    render_target_camera.render_target = Some(render_target.clone());

    let mut fun: f32 = 0.0;
    let ball_radius = 50.0;
    let mut ball_position = Vec2::new(SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2.);
    let mut ball_direction = Vec2::new(1.0, 0.0);
    ball_direction = ball_direction.rotate(Vec2::from_angle(gen_range(0., PI * 2.)));
    let mut cursor_position = Vec2::ZERO;
    let mut ball_speed = SCREEN_HEIGHT / 3.0;

    let mut bat_position = Vec2::new(SCREEN_WIDTH as f32 / 2., SCREEN_HEIGHT as f32 - 50.);
    let bat_size = Vec2::new(200., 30.);
    let mut bat_direction = Vec2::ZERO;
    let bat_speed = get_screen_data().width as f32 / 1.2;

    loop {
        let scale = f32::min(
            screen_width() / SCREEN_WIDTH,
            screen_height() / SCREEN_HEIGHT,
        );

        let dt = get_frame_time();
        set_camera(&render_target_camera);

        clear_background(WHITE);
        // input
        bat_direction = Vec2::ZERO;
        if is_key_down(KeyCode::A) {
            bat_direction.x -= 1.;
        }

        if is_key_down(KeyCode::D) {
            bat_direction.x += 1.;
        }
        bat_position += bat_direction * dt * bat_speed;

        if bat_position.x - bat_size.x / 2. <= 0. {
            bat_position.x = bat_size.x / 2.;
        }

        if bat_position.x + bat_size.x / 2. >= SCREEN_WIDTH {
            bat_position.x = SCREEN_WIDTH - bat_size.x / 2.;
        }

        ball_position += ball_direction * ball_speed * dt;
        let bat_position = bat_position - (bat_size / 2.);

        draw_circle(
            ball_position.x,
            ball_position.y,
            ball_radius,
            Color::from_rgba(12, 32, 77, 200),
        );
        draw_line(
            ball_position.x,
            ball_position.y,
            (ball_position.x + ball_direction.x),
            (ball_position.y + ball_direction.y),
            1.0,
            RED,
        );

        if ball_position.x + ball_radius > SCREEN_WIDTH + TOLERANCE {
            ball_position.x = SCREEN_WIDTH - ball_radius + TOLERANCE;
            ball_direction.x *= -1.;
        }

        if ball_position.x - ball_radius < -TOLERANCE {
            ball_position.x = ball_radius - TOLERANCE;
            ball_direction.x *= -1.;
        }

        if ball_position.y + ball_radius > SCREEN_HEIGHT + TOLERANCE {
            ball_position.y = SCREEN_HEIGHT - ball_radius + TOLERANCE;
            ball_direction.y *= -1.;
        }

        if ball_position.y - ball_radius < -TOLERANCE {
            ball_position.y = ball_radius - TOLERANCE;
            ball_direction.y *= -1.;
        }

        let ball_rect = Rect::new(
            ball_position.x - ball_radius,
            ball_position.y - ball_radius,
            ball_radius * 2.,
            ball_radius * 2.,
        );
        let bat_rect = Rect::new(bat_position.x, bat_position.y, bat_size.x, bat_position.y);
        if ball_rect.intersect(bat_rect).is_some() {
            ball_direction *= -1.;
        }

        draw_rectangle(bat_position.x, bat_position.y, bat_size.x, bat_size.y, BLUE);

        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &render_target.texture,
            (screen_width() - SCREEN_WIDTH * scale) / 2.,
            (screen_height() - SCREEN_HEIGHT * scale) / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(SCREEN_WIDTH * scale, SCREEN_HEIGHT * scale)),
                flip_y: true,
                ..Default::default()
            },
        );

        next_frame().await;
    }
}

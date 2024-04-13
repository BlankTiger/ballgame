use num_traits::ToPrimitive;
use rand::Rng;
use std::time::Duration;

use raylib_ffi::{
    colors::{DARKGRAY, RED, WHITE},
    enums::{ConfigFlags, KeyboardKey},
    BeginDrawing, ClearBackground, ClearWindowState, CloseWindow, Color, DrawCircleV, DrawText,
    EndDrawing, GetScreenHeight, GetScreenWidth, InitWindow, IsKeyDown, IsKeyPressed,
    IsWindowFullscreen, IsWindowResized, SetConfigFlags, SetExitKey, SetTargetFPS, SetWindowSize,
    Vector2, WindowShouldClose,
};

fn main() {
    unsafe { _main() };
}

unsafe fn _main() {
    let mut screen_width = 800;
    let mut screen_height = 600;
    SetConfigFlags(ConfigFlags::WindowResizable as u32);
    InitWindow(
        screen_width,
        screen_height,
        "Raylib test\0".as_ptr() as *const i8,
    );
    SetTargetFPS(60);

    let mut ball_pos = Vector2 { x: 400.0, y: 300.0 };
    let mut radius = 50.0;
    let mut speed: u8 = 5;
    let mut color = RED;
    while !WindowShouldClose() {
        if IsWindowResized() {
            screen_width = GetScreenWidth();
            screen_height = GetScreenHeight();
            SetWindowSize(screen_width, screen_height);
        }
        check_if_ball_in_bounds(&mut ball_pos, radius, screen_width, screen_height);
        modify_ball_on_input(
            &mut ball_pos,
            &mut radius,
            &mut speed,
            &mut color,
            &screen_width,
            &screen_height,
        );

        BeginDrawing();

        ClearBackground(WHITE);
        draw_instructions();
        DrawCircleV(ball_pos, radius, color);

        EndDrawing();
        // draw_hello_world();
    }
    CloseWindow();
}

unsafe fn draw_instructions() {
    DrawText(
        "Move the ball with arrow keys\0".as_ptr() as *const i8,
        10,
        10,
        20,
        DARKGRAY,
    );
    DrawText("R - grow\0".as_ptr() as *const i8, 10, 35, 20, DARKGRAY);
    DrawText("E - shrink\0".as_ptr() as *const i8, 10, 60, 20, DARKGRAY);
    DrawText("F - faster\0".as_ptr() as *const i8, 10, 85, 20, DARKGRAY);
    DrawText("D - slower\0".as_ptr() as *const i8, 10, 110, 20, DARKGRAY);
    DrawText(
        "C - taste the rainbow\0".as_ptr() as *const i8,
        10,
        135,
        20,
        DARKGRAY,
    );
}

unsafe fn modify_ball_on_input(
    ball_pos: &mut Vector2,
    radius: &mut f32,
    speed: &mut u8,
    color: &mut Color,
    width: &i32,
    height: &i32,
) {
    let max_x = *width as f32 - *radius;
    let max_y = *height as f32 - *radius;
    let min_x = *radius;
    let min_y = *radius;
    if IsKeyDown(KeyboardKey::Left.to_i32().unwrap()) && ball_pos.x > min_x {
        let new_x = ball_pos.x - *speed as f32;
        ball_pos.x = if new_x > min_x { new_x } else { min_x }
    }
    if IsKeyDown(KeyboardKey::Right.to_i32().unwrap()) && ball_pos.x < max_x {
        let new_x = ball_pos.x + *speed as f32;
        ball_pos.x = if new_x < max_x { new_x } else { max_x }
    }
    if IsKeyDown(KeyboardKey::Up.to_i32().unwrap()) && ball_pos.y > min_y {
        let new_y = ball_pos.y - *speed as f32;
        ball_pos.y = if new_y > min_y { new_y } else { min_y }
    }
    if IsKeyDown(KeyboardKey::Down.to_i32().unwrap()) && ball_pos.y < max_y {
        let new_y = ball_pos.y + *speed as f32;
        ball_pos.y = if new_y < max_y { new_y } else { max_y }
    }
    if IsKeyDown(KeyboardKey::R.to_i32().unwrap()) {
        *radius += 1.0;
    }
    if IsKeyDown(KeyboardKey::E.to_i32().unwrap()) {
        *radius -= 1.0;
    }
    if IsKeyDown(KeyboardKey::F.to_i32().unwrap()) {
        *speed = speed.checked_add(1).unwrap_or(u8::MAX);
    }
    if IsKeyDown(KeyboardKey::D.to_i32().unwrap()) {
        *speed = speed.checked_sub(1).unwrap_or(0);
    }

    if IsKeyDown(KeyboardKey::C.to_i32().unwrap()) {
        let mut rng = rand::thread_rng();
        let num_upto_10 = rng.gen_range(0..=10);
        if num_upto_10 % 3 == 0 {
            modify_to_random_color(color);
        }
    }
}

fn modify_to_random_color(color: &mut Color) {
    let mut rng = rand::thread_rng();
    color.r = rng.gen();
    color.g = rng.gen();
    color.b = rng.gen();
}

unsafe fn check_if_ball_in_bounds(ball_pos: &mut Vector2, radius: f32, width: i32, height: i32) {
    if ball_pos.x < radius {
        ball_pos.x = radius;
    }
    if ball_pos.y < radius {
        ball_pos.y = radius;
    }
    if ball_pos.x > width as f32 - radius {
        ball_pos.x = width as f32 - radius;
    }
    if ball_pos.y > height as f32 - radius {
        ball_pos.y = height as f32 - radius;
    }
}

unsafe fn draw_hello_world() {
    'outer: for x in (0..800).chain((0..800).rev()).filter(|x| x % 10 == 0) {
        for y in (0..600).chain((0..600).rev()).filter(|y| y % 10 == 0) {
            if IsKeyPressed(KeyboardKey::Escape.to_i32().unwrap()) {
                SetExitKey(0);
                break 'outer;
            }
            BeginDrawing();
            DrawText("Hello, world!".as_ptr() as *const i8, x, y, 20, DARKGRAY);
            ClearBackground(WHITE);
            EndDrawing();
        }
    }
}

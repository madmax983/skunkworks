use ::rand::Rng;
use image::{Rgba, RgbaImage};
use macroquad::prelude::*;
use std::path::PathBuf;

// Try to use miniquad from macroquad re-export
use macroquad::miniquad::window::{dropped_file_count, dropped_file_path};

mod lua_runtime;
mod stega;

use lua_runtime::LuaRuntime;

enum AppState {
    Editor(EditorState),
    Running(RunningState),
}

struct EditorState {
    cover_image: RgbaImage,
    payload: String,
    stego_image: RgbaImage,
    texture: Texture2D,
    lsb_texture: Texture2D,
    show_lsb: bool,
    seed: u64,
    message: String,
    message_timer: f32,
}

struct RunningState {
    runtime: LuaRuntime,
    script_source: String,
}

#[macroquad::main("Chromatic Cipher")]
async fn main() {
    let mut state = initialize_default_editor();

    loop {
        clear_background(BLACK);

        // Handle global inputs (File Drops handled in specific updates or here?)
        // Better here to switch states.
        if dropped_file_count() > 0 {
            if let Some(path) = dropped_file_path(0) {
                println!("File dropped: {:?}", path);
                handle_file_drop(&mut state, path);
            }
        }

        let mut next_state: Option<AppState> = None;

        match &mut state {
            AppState::Editor(editor) => {
                update_editor(editor);
                draw_editor(editor);
            }
            AppState::Running(running) => {
                update_running(running);
                draw_running(running);

                if is_key_pressed(KeyCode::Escape) {
                    // Return to editor with the script
                    let mut new_state = initialize_default_editor();
                    if let AppState::Editor(editor) = &mut new_state {
                        editor.payload = running.script_source.clone();
                        update_stego_image(editor);
                        editor.message = "Exited Runtime".to_string();
                    }
                    next_state = Some(new_state);
                }
            }
        }

        if let Some(s) = next_state {
            state = s;
        }

        next_frame().await
    }
}

fn initialize_default_editor() -> AppState {
    let width = 512;
    let height = 512;
    let mut cover = RgbaImage::new(width, height);
    let mut rng = ::rand::thread_rng();

    for pixel in cover.pixels_mut() {
        *pixel = Rgba([rng.gen(), rng.gen(), rng.gen(), 255]);
    }

    let payload = "-- LUA Payload\n-- This text is hidden in the image pixels.\n\nfunction update()\nend\n\nfunction draw()\n    local t = time()\n    draw_text('HIDDEN CODE', 100, 100 + math.sin(t)*50, 50, 0xFF00FF)\nend".to_string();

    let mut stego = cover.clone();
    let seed = 1337;
    // Embedding happens in update_stego_image

    // We need textures immediately for the state.
    // So we'll do a partial init and then call update.

    // For now, just embed once here manually to satisfy struct
    let _ = stega::embed(&mut stego, payload.as_bytes(), seed);

    let texture = Texture2D::from_rgba8(width as u16, height as u16, stego.as_raw());
    texture.set_filter(FilterMode::Nearest);

    let lsb_texture = Texture2D::from_rgba8(width as u16, height as u16, stego.as_raw()); // Placeholder
    lsb_texture.set_filter(FilterMode::Nearest);

    let mut editor = EditorState {
        cover_image: cover,
        payload,
        stego_image: stego,
        texture,
        lsb_texture,
        show_lsb: false,
        seed,
        message: "Drag & Drop Image or Script".to_string(),
        message_timer: 5.0,
    };

    update_lsb_texture(&mut editor);

    AppState::Editor(editor)
}

fn handle_file_drop(state: &mut AppState, path: PathBuf) {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext = ext.to_lowercase();
        match ext.as_str() {
            "png" | "jpg" | "jpeg" | "bmp" => {
                if let Ok(img) = image::open(&path) {
                    let img = img.to_rgba8();
                    let seed = 1337;

                    // Try to extract payload first
                    if let Ok(bytes) = stega::extract(&img, seed) {
                        if let Ok(s) = String::from_utf8(bytes) {
                            // Check if it looks like Lua
                            if s.contains("function update")
                                || s.contains("draw_text")
                                || s.contains("macroquad")
                            {
                                println!("Lua payload detected!");
                                if let Ok(runtime) = LuaRuntime::new() {
                                    if runtime.load_script(&s).is_ok() {
                                        *state = AppState::Running(RunningState {
                                            runtime,
                                            script_source: s,
                                        });
                                        return;
                                    }
                                }
                            }

                            // If not Lua or failed to load, open in Editor with extracted payload
                            let (texture, lsb_texture) = create_textures(&img);
                            *state = AppState::Editor(EditorState {
                                cover_image: img.clone(),
                                stego_image: img,
                                payload: s,
                                texture,
                                lsb_texture,
                                show_lsb: false,
                                seed,
                                message: "Payload Extracted".to_string(),
                                message_timer: 3.0,
                            });
                            return;
                        }
                    }

                    // Fallback: Load as new cover image
                    let payload = "-- New Cover Image".to_string();
                    let (texture, lsb_texture) = create_textures(&img);

                    *state = AppState::Editor(EditorState {
                        cover_image: img.clone(),
                        stego_image: img,
                        payload,
                        texture,
                        lsb_texture,
                        show_lsb: false,
                        seed,
                        message: "Cover Image Loaded".to_string(),
                        message_timer: 3.0,
                    });

                    // Embed initial payload into new cover
                    if let AppState::Editor(editor) = state {
                        update_stego_image(editor);
                    }
                }
            }
            "lua" | "txt" | "rs" => {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    match state {
                        AppState::Editor(editor) => {
                            editor.payload = content;
                            update_stego_image(editor);
                            editor.message = "Script Loaded".to_string();
                            editor.message_timer = 3.0;
                        }
                        AppState::Running(_) => {
                            // Switch to editor with default cover
                            *state = initialize_default_editor();
                            if let AppState::Editor(editor) = state {
                                editor.payload = content;
                                update_stego_image(editor);
                                editor.message = "Script Loaded (New Editor)".to_string();
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn create_textures(img: &RgbaImage) -> (Texture2D, Texture2D) {
    let width = img.width();
    let height = img.height();
    let texture = Texture2D::from_rgba8(width as u16, height as u16, img.as_raw());
    texture.set_filter(FilterMode::Nearest);
    let lsb = Texture2D::from_rgba8(width as u16, height as u16, img.as_raw());
    lsb.set_filter(FilterMode::Nearest);
    (texture, lsb)
}

fn update_editor(editor: &mut EditorState) {
    if editor.message_timer > 0.0 {
        editor.message_timer -= get_frame_time();
    }

    if is_key_pressed(KeyCode::Space) {
        editor.show_lsb = !editor.show_lsb;
    }

    if is_key_pressed(KeyCode::S) {
        let path = "output.png";
        if let Err(e) = editor.stego_image.save(path) {
            editor.message = format!("Save Error: {}", e);
        } else {
            editor.message = format!("Saved to {}", path);
        }
        editor.message_timer = 3.0;
    }

    if is_key_pressed(KeyCode::R) {
        // Try to run current payload
        if let Ok(runtime) = LuaRuntime::new() {
            if let Err(e) = runtime.load_script(&editor.payload) {
                editor.message = format!("Lua Error: {}", e);
                editor.message_timer = 5.0;
            } else {
                // We can't switch state here easily because we have &mut EditorState, not &mut AppState.
                // We need to handle this in the main loop or use a flag.
                // But for now, user must drag and drop the saved image or I need to refactor update_editor to return transition.
                editor.message = "Drag/Drop output.png to Run!".to_string();
                editor.message_timer = 5.0;
            }
        }
    }
}

fn draw_editor(editor: &EditorState) {
    let tex = if editor.show_lsb {
        &editor.lsb_texture
    } else {
        &editor.texture
    };

    let screen_w = screen_width();
    let screen_h = screen_height();
    let img_w = tex.width();
    let img_h = tex.height();

    let scale = (screen_w / img_w).min(screen_h / img_h) * 0.9;
    let dest_w = img_w * scale;
    let dest_h = img_h * scale;
    let dest_x = (screen_w - dest_w) / 2.0;
    let dest_y = (screen_h - dest_h) / 2.0;

    draw_texture_ex(
        tex,
        dest_x,
        dest_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(dest_w, dest_h)),
            ..Default::default()
        },
    );

    // UI
    draw_text("CHROMATIC CIPHER", 20.0, 30.0, 40.0, WHITE);
    draw_text("[SPACE] Toggle LSB View", 20.0, 60.0, 20.0, GRAY);
    draw_text("[S] Save 'output.png'", 20.0, 80.0, 20.0, GRAY);
    draw_text("[R] Test Syntax", 20.0, 100.0, 20.0, GRAY);

    if editor.show_lsb {
        draw_text("MODE: LSB VIEW", 20.0, screen_h - 50.0, 30.0, RED);
    }

    if editor.message_timer > 0.0 {
        draw_text(&editor.message, 20.0, screen_h - 20.0, 30.0, GREEN);
    }
}

fn update_running(running: &mut RunningState) {
    let _ = running.runtime.call_update();
}

fn draw_running(running: &RunningState) {
    let _ = running.runtime.call_draw();
}

fn update_stego_image(editor: &mut EditorState) {
    editor.stego_image = editor.cover_image.clone();
    if let Err(e) = stega::embed(
        &mut editor.stego_image,
        editor.payload.as_bytes(),
        editor.seed,
    ) {
        editor.message = format!("Embed Error: {}", e);
        editor.message_timer = 5.0;
    } else {
        editor.message = "Payload Embedded".to_string();
        editor.message_timer = 2.0;
    }

    // Update textures
    editor.texture = Texture2D::from_rgba8(
        editor.stego_image.width() as u16,
        editor.stego_image.height() as u16,
        editor.stego_image.as_raw(),
    );
    editor.texture.set_filter(FilterMode::Nearest);

    update_lsb_texture(editor);
}

fn update_lsb_texture(editor: &mut EditorState) {
    let mut lsb_view_img = editor.stego_image.clone();
    for pixel in lsb_view_img.pixels_mut() {
        pixel[0] = (pixel[0] & 1) * 255;
        pixel[1] = (pixel[1] & 1) * 255;
        pixel[2] = (pixel[2] & 1) * 255;
        pixel[3] = 255;
    }
    editor.lsb_texture = Texture2D::from_rgba8(
        lsb_view_img.width() as u16,
        lsb_view_img.height() as u16,
        lsb_view_img.as_raw(),
    );
    editor.lsb_texture.set_filter(FilterMode::Nearest);
}

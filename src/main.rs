mod framebuffer;
mod line;
mod obj;

use raylib::prelude::*;

use framebuffer::Framebuffer;
use line::triangle;
use obj::{Obj, load_obj};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
/// Fracción de la ventana que puede ocupar el modelo (deja un margen).
const FILL: f32 = 0.8;

/// Escala y traslación que llevan las coordenadas (x, y) del modelo a la pantalla.
struct ScreenFit {
    scale: f32,
    model_center: Vector2,
    screen_center: Vector2,
}

impl ScreenFit {
    /// Calcula la caja que encierra todos los vértices en x e y,
    /// y elige una escala para que quepa en la ventana conservando la proporción.
    fn new(obj: &Obj, width: i32, height: i32) -> Self {
        let mut min = Vector2::new(f32::MAX, f32::MAX);
        let mut max = Vector2::new(f32::MIN, f32::MIN);
        for v in &obj.vertices {
            min.x = min.x.min(v.x);
            min.y = min.y.min(v.y);
            max.x = max.x.max(v.x);
            max.y = max.y.max(v.y);
        }

        let model_width = (max.x - min.x).max(f32::EPSILON);
        let model_height = (max.y - min.y).max(f32::EPSILON);
        let scale = (width as f32 * FILL / model_width).min(height as f32 * FILL / model_height);

        ScreenFit {
            scale,
            model_center: Vector2::new((min.x + max.x) / 2.0, (min.y + max.y) / 2.0),
            screen_center: Vector2::new(width as f32 / 2.0, height as f32 / 2.0),
        }
    }

    /// Usa solo x e y del vértice. La y se invierte porque en el modelo
    /// crece hacia arriba y en la pantalla crece hacia abajo.
    fn to_screen(&self, v: Vector3) -> Vector2 {
        Vector2::new(
            self.screen_center.x + (v.x - self.model_center.x) * self.scale,
            self.screen_center.y - (v.y - self.model_center.y) * self.scale,
        )
    }
}

/// Recorre el modelo de 3 en 3 índices y dibuja cada triángulo.
fn render_model(fb: &mut Framebuffer, obj: &Obj, fit: &ScreenFit) {
    for tri in obj.indices.chunks_exact(3) {
        // 1. Los tres índices del triángulo.
        let (i0, i1, i2) = (tri[0], tri[1], tri[2]);

        // 2. Buscar los vértices con esos índices.
        let v0 = obj.vertices[i0];
        let v1 = obj.vertices[i1];
        let v2 = obj.vertices[i2];

        // 3. Pasarlos a pantalla y dibujar el triángulo.
        triangle(fb, fit.to_screen(v0), fit.to_screen(v1), fit.to_screen(v2));
    }
}

fn main() {
    // Uso: cargo run --release -- [ruta.obj] [--screenshot salida.png]
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut model_path = String::from("assets/teapot.obj");
    let mut screenshot: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        if args[i] == "--screenshot" {
            screenshot = Some(args.get(i + 1).cloned().unwrap_or("screenshot.png".into()));
            i += 2;
        } else {
            model_path = args[i].clone();
            i += 1;
        }
    }

    let obj = match load_obj(&model_path) {
        Ok(obj) => obj,
        Err(e) => {
            eprintln!("Error al cargar el modelo: {e}");
            std::process::exit(1);
        }
    };
    println!(
        "Modelo '{}': {} vértices, {} índices, {} triángulos",
        model_path,
        obj.vertices.len(),
        obj.indices.len(),
        obj.triangle_count()
    );

    let background = Color::new(20, 20, 30, 255);
    let mut fb = Framebuffer::new(WIDTH, HEIGHT, background);
    let fit = ScreenFit::new(&obj, WIDTH, HEIGHT);

    fb.clear();
    fb.set_current_color(Color::new(255, 200, 80, 255));
    render_model(&mut fb, &obj, &fit);

    // Modo sin ventana: solo guarda la imagen y termina.
    if let Some(path) = screenshot {
        fb.save_png(&path);
        println!("Captura guardada en {path}");
        return;
    }

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Proyecto 1 - Modelo OBJ")
        .build();
    rl.set_target_fps(60);

    // El modelo no cambia, así que basta con subir la imagen una vez.
    let texture = rl
        .load_texture_from_image(&thread, &fb.to_image())
        .expect("No se pudo crear la textura");

    while !rl.window_should_close() {
        // Tecla S: guardar captura desde la ventana.
        if rl.is_key_pressed(KeyboardKey::KEY_S) {
            fb.save_png("screenshot.png");
            println!("Captura guardada en screenshot.png");
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(background);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}

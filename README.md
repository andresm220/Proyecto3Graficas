🌌 Solar System Renderer — Rust (CPU Software Renderer)

Este proyecto implementa un renderizador 3D por software escrito completamente en Rust, sin usar OpenGL, WebGL o GPU.
Toda la escena se rasteriza píxel por píxel directamente en la CPU.

Incluye:

🌞 Sol con shader procedural

🪐 Cuatro planetas con shaders distintos (rocoso, gaseoso, helado, volcánico)

🌌 Skybox procedural con estrellas

🚀 Nave 3D cargada desde un archivo OBJ (modelada en Blender)

📡 Warp instantáneo hacia el Sol y planetas

🛑 Colisiones entre cámara y cuerpos celestes

🎥 Movimiento 3D completo de cámara

✨ Modo bonito (shaders) y modo rápido (discos sólidos)

📹 Video de demostración
https://youtu.be/UPghVVlNZWw


🚀 Características principales

Renderizado completo en CPU (sin GPU)

Shaders procedurales para planetas y estrella

Skybox dinámico con miles de estrellas

Cámara con movimiento 3D: avanzar, retroceder, strafe, subir/bajar

Warp animado hacia cuerpos del sistema (1–5)

Nave OBJ siempre visible frente a la cámara

Colisiones suaves entre cámara y planetas

Órbitas dibujadas en 3D

🎮 Controles
Tecla	Acción
W / S	Avanzar / retroceder
A / D	Strafe izquierda / derecha
Espacio / Ctrl	Subir / bajar
1–5	Warp al Sol o planetas
T	Activar/desactivar animación de warp
P	Modo bonito ↔ modo rápido
ESC	Salir

📦 Cómo correr el proyecto
Compilar en modo release :
cargo run --release

📁 Estructura del proyecto

Basado en tu estructura real:

src/
 ├── bodies/
 │    ├── body.rs
 │    ├── moon.rs
 │    └── mod.rs
 ├── camera.rs
 ├── draw.rs
 ├── hud.rs
 ├── main.rs
 ├── math.rs
 ├── obj_loader.rs
 ├── orbits.rs
 ├── projector.rs
 ├── shading.rs
 ├── ship.rs
 ├── skybox.rs
 └── window.rs
Cargo.toml
Cargo.lock
nave_andres.obj   ← aquí está tu modelo 3D
.gitignore

🧠 Funcionamiento interno
Render 3D por software

El renderer realiza:

Transformación world → camera → screen

Proyección perspectiva manual

Shading procedural por píxel

Raster de la nave OBJ en modo wireframe optimizado

Orden de dibujado por distancia (Painter’s Algorithm)

Shading procedural

Cada planeta genera su superficie en tiempo real mediante:

FBM (Fractal Brownian Motion)

Gradientes hemisféricos

Bandas, turbulencia y ruido 3D

Oscurecimiento por ángulo

Warp System

Warp directo a la posición del Sol/planetas con animación suave.


✔ Requerimientos del curso cumplidos

 Estética (30 pts)

 Performance adecuado (20 pts)

 5 cuerpos celestes (50 pts)

 Warp instantáneo (10 pts)

 Warp animado (10 pts)

 Nave modelada por el estudiante (30 pts)

 Skybox procedural (10 pts)

 Colisiones (10 pts)

 Movimiento 3D (40 pts)

 Órbitas renderizadas (20 pts)





Más lunas y anillos complejos

Sombreado avanzado basado en luz del Sol

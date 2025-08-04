#version 100
attribute vec3 position;
attribute vec2 texcoord;

uniform vec2 screen_size;
varying vec2 uv;

void main() {
    uv = texcoord;

    // Convertit la position (en pixels) en clip-space [-1, 1]
    vec2 clip_pos = (position.xy / screen_size) * 2.0 - 1.0;
    clip_pos.y = -clip_pos.y; // Inversion verticale pour OpenGL

    gl_Position = vec4(clip_pos, 0.0, 1.0);
}
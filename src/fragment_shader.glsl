#version 100
precision mediump float;

varying vec2 uv;
uniform float time;

// Fonction pseudo-aléatoire
float random(vec2 st) {
    return fract(sin(dot(st, vec2(12.9898, 78.233))) * 43758.5453);
}

void main() {
    // Crée une grille virtuelle 100x100
    vec2 cell = floor(uv * 200.0);

    float r = random(cell);

    if (r > 0.99) {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0); // Pixel blanc
    } else {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0); // Fond noir
    }
}
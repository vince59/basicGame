#ifdef GL_ES
precision highp float;
#endif

uniform float direction_modifier;

#define NUM_LAYERS 4.

mat2 Rot(float a) {
    float s = sin(a), c = cos(a);
    return mat2(c, -s, s, c);
}

float Star(vec2 uv, float flare) {
    float d = length(uv);
    float m = 0.05 / d;

    float rays = max(0.0, 1.0 - abs(uv.x * uv.y * 1000.0));
    m += rays * flare;

    uv *= Rot(3.1415 / 4.0);
    rays = max(0.0, 1.0 - abs(uv.x * uv.y * 1000.0));
    m += rays * 0.3 * flare;

    m *= smoothstep(1.0, 0.2, d);
    return m;
}

float Hash21(vec2 p) {
    p = fract(p * vec2(123.34, 456.21));
    p += dot(p, p + 45.32);
    return fract(p.x * p.y);
}

vec3 StarLayer(vec2 uv, float time) {
    vec3 col = vec3(0.0);
    vec2 gv = fract(uv) - 0.5;
    vec2 id = floor(uv);

    for (int y = -1; y <= 1; y++) {
        for (int x = -1; x <= 1; x++) {
            vec2 offs = vec2(x, y);
            float n = Hash21(id + offs);
            float size = fract(n * 345.32);
            float star = Star(gv - offs - vec2(n, fract(n * 42.0)) + 0.5,
                              smoothstep(0.9, 1.0, size) * 0.6);
            vec3 color = sin(vec3(0.8) * fract(n * 2345.2) * 123.2) * 0.5 + 0.5;
            color *= vec3(0.25, 0.25, 0.20);
            star *= sin(time * 3.0 + n * 6.2831) * 0.5 + 1.0;
            col += star * size * color;
        }
    }
    return col;
}

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = (fragCoord - 0.5 * iResolution.xy) / iResolution.y;

    float t = iTime * 0.02;
    float speed = 3.0;
    vec2 direction = vec2(-0.25 + direction_modifier, -1.0) * speed;
    uv += direction;

    vec3 col = vec3(0.0);
    for (float i = 0.0; i < 1.0; i += 1.0 / NUM_LAYERS) {
        float depth = fract(i + t);
        float scale = mix(20.0, 0.5, depth);
        float fade = depth * smoothstep(1.0, 0.9, depth);
        col += StarLayer(uv * scale + i * 453.2, iTime) * fade;
    }

    fragColor = vec4(col, 1.0);
}

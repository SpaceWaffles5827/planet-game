#version 150

in vec2 v_uv;
in vec4 v_color;

uniform sampler2D u_texture;
uniform sampler2D u_overlay;

out vec4 o_color;

void main() {
    o_color = v_color * texture(u_overlay, v_uv) * vec4(v_uv.x, 0.0, 1.0, 1.0);
}
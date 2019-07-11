#version 330

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;
uniform vec3 tint;

void main() {
    color = vec4(tint, 1.0) * texture(tex, v_tex_coords);
}

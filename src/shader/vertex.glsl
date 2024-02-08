#version 130
in vec3 position;
in vec2 texcoord;
in vec4 color0;

uniform mat4 Model;
uniform mat4 Projection;

out vec2 uv;

void main() {
    uv = texcoord;
    gl_Position = Projection * Model * vec4(position, 1);
}
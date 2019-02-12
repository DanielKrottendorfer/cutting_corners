#version 460

in vec3 position;
in vec3 normal;
in vec2 texture;

out vec2 UV;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

void main() {
    mat4 modelview = view * model;
    gl_Position = perspective * modelview * vec4(position, 1.0);
    UV = texture;
}
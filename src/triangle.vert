#version 330 core
layout (location = 0) in vec2 pos;
uniform mat4 model;
uniform vec3 color;
out vec3 fragcolor;
void main(){
    gl_Position = model * vec4(pos, 0.0, 1.0);
    fragcolor = color;
}

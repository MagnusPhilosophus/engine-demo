#version 330 core
in vec3 fragcolor;
out vec4 color;

void main(){
    color = vec4(fragcolor, 1.0f);
}

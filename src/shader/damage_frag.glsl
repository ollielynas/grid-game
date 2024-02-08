#version 130
precision lowp float;
uniform vec4 _Time;
uniform vec2 ScreenSize;
uniform float Damage;
uniform sampler2D Texture;

in vec2 uv;

out vec4 color;



void main() {

    vec2 px = uv*ScreenSize;

    float distance = min(min(px.x, px.y), min(ScreenSize.x - px.x, ScreenSize.y - px.y)) / (ScreenSize.x/500.0);

    
    color = vec4(0.9608, 0.3137, 0.3137, 0.299);
    color.a = color.a / distance*Damage;



}
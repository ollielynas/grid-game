#version 130
precision lowp float;
uniform vec2 textureSize;
uniform vec4 _Time;
uniform sampler2D Texture;

in vec2 uv;

out vec4 color;


void main() {
    float time = _Time.x;
    vec2 pixelCoord = uv * textureSize;

    vec2 o1 = vec2(1.0, 0.0) / textureSize;
    vec2 o2 = vec2(0.0, 1.0) / textureSize;

    vec4 c2 = texture(Texture, uv);


    color = c2 / 2.0;
    //color = texture(Texture, coord / textureSize);

    //color = vec4(0.0, 0.0, 0.0, 0.0);
}
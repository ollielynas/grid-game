#version 130

in vec4 color;
in vec2 uv;

uniform sampler2D Texture;
uniform vec2 ScreenSize;
uniform vec4 _Time;

out vec4 fragColor;

const float range = 0.05;
const float noiseQuality = 250.0;
const float noiseIntensity = 0.00088;
const float offsetIntensity = 0.002;
const float colorOffsetIntensity = 0.13;

float rand(vec2 co)
{
    return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

float verticalBar(float pos, float uvY, float offset)
{
    float edge0 = (pos - range);
    float edge1 = (pos + range);

    float x = smoothstep(edge0, pos, uvY) * offset;
    x -= smoothstep(pos, edge1, uvY) * offset;
    return x;
}

void main() {
    float time = _Time.x;
    vec2 modifiedUV = vec2(uv.x, 1.0 - uv.y);

    for (float i = 0.0; i < 0.71; i += 0.1313)
    {
        float d = mod(time * i, 1.7);
        float o = sin(1.0 - tan(time * 0.24 * i));
    	o *= offsetIntensity;
        modifiedUV.x += verticalBar(d, modifiedUV.y, o);
    }

    float uvY = modifiedUV.y;
    uvY *= noiseQuality;
    uvY = float(int(uvY)) * (1.0 / noiseQuality);
    float noise = rand(vec2(time * 0.00001, uvY));
    modifiedUV.x += noise * noiseIntensity;

    vec2 offsetR = vec2(0.006 * sin(time), 0.0) * colorOffsetIntensity;
    vec2 offsetG = vec2(0.0073 * (cos(time * 0.97)), 0.0) * colorOffsetIntensity;
    
    float r = texture(Texture, modifiedUV + offsetR).r;
    float g = texture(Texture, modifiedUV + offsetG).g;
    float b = texture(Texture, modifiedUV).b;

    fragColor = vec4(r, g, b, 1.0);
}
#version 130

in vec4 color;
in vec2 uv;

uniform sampler2D Texture;
uniform vec2 ScreenSize;
uniform vec4 _Time;
uniform float healthPercent;

out vec4 fragColor;

const float range = 0.05;
const float noiseQuality = 250.0;
const float noiseIntensity = 0.00088;
const float offsetIntensity = 0.002;
const float colorOffsetIntensity = 0.13;

float healthMultiplier() {
    return 1.1 / (healthPercent + 0.1);
}


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
    vec2 unmodifiedUV = vec2(uv.x, uv.y);

    for (float i = 0.0; i < 0.71; i += 0.1313)
    {
        float d = mod(time * i, 1.7);
        float o = sin(1.0 - tan(time * 0.24 * i));
    	o *= offsetIntensity * healthMultiplier();
        unmodifiedUV.x += verticalBar(d, unmodifiedUV.y, o);
    }

    float uvY = unmodifiedUV.y;
    uvY *= noiseQuality;
    uvY = float(int(uvY)) * (1.0 / noiseQuality);

    float aspectRatio = ScreenSize.x / ScreenSize.y;

    float uvX = unmodifiedUV.x * aspectRatio;
    uvX *= noiseQuality;
    uvX = float(int(uvX)) * (1.0 / (noiseQuality * aspectRatio));

    float noise = rand(vec2(time * 0.00001, uvY));
    unmodifiedUV.x += noise * noiseIntensity * healthMultiplier();

    vec2 offsetR = vec2(0.006 * sin(time), 0.0) * colorOffsetIntensity * healthMultiplier();
    vec2 offsetG = vec2(0.0073 * (cos(time * 0.97)), 0.0) * colorOffsetIntensity * healthMultiplier();
    
    float r = texture(Texture, unmodifiedUV + offsetR).r;
    float g = texture(Texture, unmodifiedUV + offsetG).g;
    float b = texture(Texture, unmodifiedUV).b;

    float whiteNoiseIntensity = (1.0 - healthPercent) * (1.0 - healthPercent);

    float whiteNoise = whiteNoiseIntensity * (rand(vec2(uvX + time, uvY + time + 3.4)) * 0.3 - 0.15);
    fragColor = vec4(vec3(r, g, b) + whiteNoise, 1.0);
}
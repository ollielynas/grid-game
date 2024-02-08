#version 130
precision lowp float;
uniform vec2 textureSize;
uniform vec4 _Time;
uniform sampler2D Texture;

in vec2 uv;

out vec4 color;

vec3 hsb2rgb( in vec3 c ){
    vec3 rgb = clamp(abs(mod(c.x*6.0+vec3(0.0,4.0,2.0),
                             6.0)-3.0)-1.0,
                     0.0,
                     1.0 );
    rgb = rgb*rgb*(3.0-2.0*rgb);
    return c.z * mix(vec3(1.0), rgb, c.y);
}

void main() {
    float time = _Time.x;
    //vec2 coord = gl_FragCoord.xy / canvasSize.xy; // [0,1]
    vec2 pixelCoord = uv * textureSize;
    //vec2 t = mod(pixelCoord, 1.0);
    //t = t * t * t * (t * (6.0 * t - 15.0) + 10.0);
    //vec2 coord = mix(floor(pixelCoord), floor(pixelCoord) + 1.0, t);
    
    vec2 o1 = vec2(1.0, 0.0) / textureSize;
    vec2 o2 = vec2(0.0, 1.0) / textureSize;

    vec4 c2 = texture(Texture, uv);
    c2.a = 1.0-c2.a;

    color = vec4(0.0, 0.0, 0.0, 1.0) * (texture(Texture, uv) * 4.0 + texture(Texture, uv + o1) + texture(Texture, uv + o2) + texture(Texture, uv - o1) + texture(Texture, uv - o2)).a * 0.125;
    color = color;
    //color = texture(Texture, coord / textureSize);

    //color = vec4(0.0, 0.0, 0.0, 0.0);
}
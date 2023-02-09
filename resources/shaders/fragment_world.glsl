#version 330 core
out vec4 FragColor;

in vec2 texCoord;

uniform sampler2D ourTexture;
uniform vec3 fgColor;
uniform vec3 bgColor;
uniform float light;

void main()
{
    // As a black-to-white bitmap, the value of any color channel is the value of all of them...
    float intensity = texture(ourTexture, texCoord).r;

    FragColor = mix(vec4(bgColor, 1.0), vec4(fgColor, 1.0), intensity) * light;
}
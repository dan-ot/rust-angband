#version 330 core
out vec4 FragColor;

in vec2 texCoord;

uniform sampler2D ourTexture;
uniform float light;

void main()
{
    // As a black-to-white bitmap, the value of any color channel is the value of all of them...
    vec4 color = texture(ourTexture, texCoord);
    float light_val = light / 2.0;

    FragColor = vec4(color.r * light_val, color.g * light_val, color.b * light_val, color.a);
}
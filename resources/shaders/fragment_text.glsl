#version 330 core
out vec4 FragColor;

in vec2 texCoord;

uniform sampler2D ourTexture;
uniform vec3 fgColor;

void main()
{
    // As a black-to-white bitmap, the value of any color channel is the value of all of them...
    float intensity = texture(ourTexture, texCoord).r;

    vec4 maybeOut = vec4(fgColor, intensity);
    if (maybeOut.a < 0.5) {
        discard;
    } else {
        FragColor = maybeOut;
    }
}
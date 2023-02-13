#version 450 core
out vec4 FragColor;

in vec4 gl_FragCoord;
in vec2 texCoord;

uniform sampler2D ourTexture;

void main()
{
    // As a black-to-white bitmap, the value of any color channel is the value of all of them...
    vec3 original = texture(ourTexture, texCoord).rgb;
    float avg = (original.r + original.b + original.g) / 3.0;
    vec3 intensity = vec3(avg) / 2.0;

    vec2 frag_position = gl_FragCoord.xy;
    int frag_x = int(round(frag_position.x)) / 4;
    int frag_y = int(round(frag_position.y)) / 4;

    if (frag_x % 2 == 1
        || frag_y % 2 == 1) {
        FragColor = vec4(0.0);
    } else {
        FragColor = vec4(intensity, 1.0);
    }

}
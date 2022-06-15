#version 450

layout(location = 0) in vec3 fragColour;
layout(location = 0) out vec4 outColour;

float[3] rgb(float r, float g, float b) {
    float[3] colours = {r / 255.0, g / 255.0, b / 255.0};
    return colours;
}

void main() {
    float colours[3] = rgb(fragColour[0], fragColour[1], fragColour[2]);
    //float window_colours[3] = rgb(100, 20, 35);

    outColour = vec4(colours[0], colours[1], colours[2], 1.0); 
}


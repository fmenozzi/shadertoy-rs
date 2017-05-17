#version 150 core

uniform float iGlobalTime;
uniform vec3  iResolution;
uniform vec4  iMouse;
uniform int   iFrame;

in vec2 fragCoord;
out vec4 fragColor;

void main() {
    vec2 uv = fragCoord.xy / iResolution.xy;

    fragColor = vec4(uv, 0.5 + 0.5*sin(iGlobalTime), 1.0);

    if (distance(iMouse.xy, fragCoord.xy) <= 10.0) {
        fragColor = vec4(vec3(0.0), 1.0);
    }
}

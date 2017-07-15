void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = fragCoord.xy / iResolution.xy;

    fragColor = vec4(uv, 0.5 + 0.5*sin(iTime), 1.0);

    if (distance(iMouse.xy, fragCoord.xy) <= 10.0) {
        fragColor = vec4(vec3(0.0), 1.0);
    }
}

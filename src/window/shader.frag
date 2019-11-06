#version 450

layout(location = 0) out vec4 outColor;

layout(location = 0) in vec2 coord;

layout(set = 0, binding = 0) uniform Locals {
    mat3 cam;
    vec3 pos;
    float ratio;
    float tan_angle;
};

void main() {
    vec2 vratio = vec2(ratio,1.0);
    if (ratio < 1.0) vratio = vec2(1.0, 1.0/ratio);
    vec3 dir = vec3(coord*vratio*tan_angle, 1.0);
    dir = cam*dir/length(dir);

    //outColor = vec4((dir+1.0)/2, 1.0);
    outColor = vec4(distance(dir.xy,coord), 0.0, 0.0, 1.0);
}

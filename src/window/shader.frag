#version 450

layout(location = 0) out vec4 outColor;

layout(location = 0) in vec2 coord;

layout(set = 0, binding = 0) uniform Locals {
    mat3 cam;
    vec3 pos;
    float ratio;
    float tan_angle;
};

vec3 ray_trace(vec3 dir, vec3 pos) {
    if (distance(dir.xy,coord) < 0.01) 
        return vec3(1.0, 0.0, 0.0);
    else
        return vec3(0.0, 0.0, 0.0);
}

void main() {
    vec2 vratio = vec2(ratio,1.0);
    if (ratio < 1.0) vratio = vec2(1.0, 1.0/ratio);
    vec3 dir = vec3(coord*vratio*tan_angle, 1.0);
    dir = cam*dir/length(dir);

    outColor = vec4(ray_trace(dir, pos), 1.0);
}

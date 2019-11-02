#version 450

layout(location = 0) out vec4 outColor;

layout(location = 0) in vec2 v_TexCoord;

layout(set = 0, binding = 0) uniform Locals {
    mat3 cam;
    vec3 pos;
    float ratio;
};

void main() {
    vec3 dir = vec3(v_TexCoord, 1.0);
    dir = cam*dir/length(dir);

    outColor = vec4((dir+1.0)/2, 1.0);
}

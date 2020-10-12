#version 330 core
layout(location = 0) in vec3 aPosition;
layout(location = 1) in uint aColorIndex;
layout(location = 2) in vec3 aNormal;

out vec3 FragPos;
out vec3 Color;
out vec3 Normal;

uniform mat4 model;
uniform mat3 normalMatrix;
uniform mat4 view;
uniform mat4 projection;
// TODO: A vec3 could be used
uniform vec4 palette[256];

void main() {
    gl_Position = projection * view * model * vec4(aPosition, 1.0f);
    FragPos = vec3(model * vec4(aPosition, 1.0));
    Color = palette[aColorIndex].xyz;
    Normal = normalMatrix * aNormal;
}

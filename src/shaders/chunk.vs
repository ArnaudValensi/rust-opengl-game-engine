#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aNormal;
layout (location = 2) in vec2 aTexCoord;

out vec4 Color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec4 colors[256];

void main()
{
	gl_Position = projection * view * model * vec4(aPos, 1.0f);
	Color = colors[6];
}
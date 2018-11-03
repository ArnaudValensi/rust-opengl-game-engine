#version 330 core
layout (location = 0) in vec3 Position;
layout (location = 1) in uint ColorIndex;
layout (location = 2) in vec2 Normal;
layout (location = 3) in vec2 TexCoord;

out vec4 Color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec4 palette[256];

void main()
{
	gl_Position = projection * view * model * vec4(Position, 1.0f);
	Color = palette[ColorIndex];
}

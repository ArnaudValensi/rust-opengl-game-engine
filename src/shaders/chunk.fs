#version 330 core
out vec4 FragColor;

in vec4 Color;

// texture samplers
uniform sampler2D texture1;
uniform sampler2D texture2;

void main()
{
	FragColor = Color;
}

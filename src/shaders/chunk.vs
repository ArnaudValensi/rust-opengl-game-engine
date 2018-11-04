#version 330 core
layout (location = 0) in vec3 aPosition;
layout (location = 1) in uint ColorIndex;
layout (location = 2) in vec3 aNormal;
layout (location = 3) in vec2 TexCoord;

out vec3 FragPos;
out vec3 Color;
out vec3 Normal;

uniform mat4 model;
uniform mat3 normalMatrix;
uniform mat4 view;
uniform mat4 projection;
uniform vec4 palette[256];

vec3 lightColor = vec3(1.0f, 1.0f, 1.0f);
// vec3 lightPos = vec3(1.2f, 1.0f, 2.0f);

void main()
{
	gl_Position = projection * view * model * vec4(aPosition, 1.0f);
	FragPos = vec3(model * vec4(aPosition, 1.0));
	Color = palette[ColorIndex].xyz * lightColor;
	Normal = normalMatrix * aNormal;
}

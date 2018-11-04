#version 330 core

in vec3 Color;
out vec4 FragColor;

vec3 lightColor = vec3(1.0f, 1.0f, 1.0f);

void main()
{
	float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    vec3 result = ambient * Color;
    FragColor = vec4(result, 1.0);
}

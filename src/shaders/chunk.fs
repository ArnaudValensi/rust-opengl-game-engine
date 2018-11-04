#version 330 core

in vec3 FragPos;
in vec3 Color;
in vec3 Normal;

out vec4 FragColor;

uniform vec3 ambientLightColor;
uniform vec3 lightPosition;
uniform vec3 lightColor;

void main()
{
	float ambientStrength = 0.1;

	// Ambient light calculation
    vec3 ambient = ambientStrength * ambientLightColor;

	// Diffuse light calculation
	vec3 norm = normalize(Normal);
	vec3 lightDir = normalize(lightPosition - FragPos);
	float diff = max(dot(norm, lightDir), 0.0);
	vec3 diffuse = diff * lightColor;

	vec3 result = (ambient + diffuse) * Color;
    FragColor = vec4(result, 1.0);
}

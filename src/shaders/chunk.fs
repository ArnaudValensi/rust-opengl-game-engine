#version 330 core

in vec3 FragPos;
in vec3 Color;
in vec3 Normal;

out vec4 FragColor;

vec3 ambientLightColor = vec3(1.0f, 1.0f, 1.0f);
vec3 lightColor = vec3(1.0f, 1.0f, 1.0f);

uniform vec3 lightPos;

void main()
{
	float ambientStrength = 0.1;

	// Ambient light calculation
    vec3 ambient = ambientStrength * ambientLightColor;

	// Diffuse light calculation
	vec3 norm = normalize(Normal);
	vec3 lightDir = normalize(lightPos - FragPos);
	float diff = max(dot(norm, lightDir), 0.0);
	vec3 diffuse = diff * lightColor;

	vec3 result = (ambient + diffuse) * Color;
    FragColor = vec4(result, 1.0);
}

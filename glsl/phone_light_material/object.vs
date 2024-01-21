#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

out vec3 FragPos;
out vec3 Normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(aPos, 1.0);

    // 计算物体表面某一点在世界空间的位置向量
    FragPos = vec3(model * vec4(aPos, 1.0));
    
    Normal = mat3(transpose(inverse(model))) * aNormal;
}
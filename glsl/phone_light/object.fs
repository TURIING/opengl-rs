#version 330 core

out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;

uniform vec3 objectColor;
uniform vec3 viewPos;               // 摄像机的位置向量
uniform vec3 lightColor;            
uniform vec3 lightPos;              // 光源的位置向量


void main() {
    /* 环境光照 */
    float ambientStrength = 0.1;                                            // 环境光照的颜色强度
    vec3 ambient = ambientStrength * lightColor;                            // 环境光照分量

    /* 漫反射光照 */
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);                          // 光的方向向量，从物体表面的某一点指向光源
    float diffuseStrength = max(dot(norm, lightDir), 0.0);                  // 漫反射光照的颜色强度
    vec3 diffuse = diffuseStrength * lightColor;                            // 漫反射光照分量

    /* 镜面光照 */
    float specularStrength = 0.5;                                           // 镜面光照的强度系数
    vec3 viewDir = normalize(viewPos - FragPos);                            // 观察向量
    vec3 reflectDir = reflect(-lightDir, norm);                             // 反射向量，reflect 函数要求第一个向量是从光源指向片段位置的向量，但是 lightDir 当前正好相反，所以需要取反
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * lightColor;                   // 镜面光照分量

    /* 计算最终颜色 */
    vec3 result = (ambient + diffuse + specular) * objectColor;
    FragColor = vec4(result, 1.0);
}
#version 330 core

/* 定义材质属性 */
struct Material {
    vec3 ambient;                                                           // 在环境光照下，表面反射的颜色，通常与表面的颜色相同
    vec3 diffuse;                                                           // 在漫反射光照下，表面的颜色
    vec3 specular;                                                          // 镜面反射下，高光的颜色
    float shininess;                                                        // 反光度，镜面高光的散射/半径
};

/* 定义光源属性 */
struct Light {
    vec3 position;

    vec3 ambient;                                                           // 环境光照的颜色强度
    vec3 diffuse;                                                           // 漫反射光照的颜色强度
    vec3 specular;                                                          // 镜面光照的颜色强度
};

out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;

uniform vec3 viewPos;                                                       // 摄像机的位置向量
uniform Material material;
uniform Light light;

void main() {
    /* 环境光照 */
    vec3 ambient = light.ambient * material.ambient;                        // 环境光照分量

    /* 漫反射光照 */
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPos);                          // 光的方向向量，从物体表面的某一点指向光源
    float diffuseStrength = max(dot(norm, lightDir), 0.0);                  // 漫反射光照的颜色强度
    vec3 diffuse = (diffuseStrength * material.diffuse) * light.diffuse;    // 漫反射光照分量

    /* 镜面光照 */
    vec3 viewDir = normalize(viewPos - FragPos);                            // 观察向量
    vec3 reflectDir = reflect(-lightDir, norm);                             // 反射向量，reflect 函数要求第一个向量是从光源指向片段位置的向量，但是 lightDir 当前正好相反，所以需要取反
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = (spec * material.specular) * light.specular;            // 镜面光照分量

    /* 计算最终颜色 */
    vec3 result = ambient + diffuse + specular;
    FragColor = vec4(result, 1.0);
}
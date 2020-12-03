#version 450

const int MAX_LIGHTS = 10;

struct Light {
    mat4 proj;
    vec4 pos;
    vec4 color;
};

layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Normal;

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Lights {
    vec3 AmbientColor;
    uvec4 NumLights;
    Light SceneLights[MAX_LIGHTS];
};

layout(set = 3, binding = 0) uniform ArrayMaterial_albedo {
    vec4 Albedo;
};

# ifdef ARRAYMATERIAL_ALBEDO_TEXTURE
layout(set = 3, binding = 1) uniform texture2D ArrayMaterial_albedo_texture;
layout(set = 3, binding = 2) uniform sampler ArrayMaterial_albedo_texture_sampler;
# endif

void main() {
    vec4 output_color = Albedo;

    vec3 normal = normalize(v_Normal);
    // accumulate color
    vec3 color = AmbientColor;
    for (int i=0; i<int(NumLights.x) && i<MAX_LIGHTS; ++i) {
        Light light = SceneLights[i];
        // compute Lambertian diffuse term
        vec3 light_dir = normalize(light.pos.xyz - v_Position);
        float diffuse = max(0.0, dot(normal, light_dir));
        // add light contribution
        color += diffuse * light.color.rgb;
    }
    output_color.rgb *= color;

    // multiply the light by material color
    o_Target = output_color;
}

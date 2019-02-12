#version 460

const int MAX_JOINTS = 50;//max joints allowed in a skeleton
in vec3 position;
in vec3 normal;
in vec2 texture;
in vec4 weights;
in ivec4 joint_mi;
in int joint_c;

out vec2 UV;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

layout(column_major) buffer MyBlock {
      mat4 transform_m[64];
};

void main() {
    vec4 accPosition = vec4(0,0,0,0);

    for( int i = 0;i<joint_c;i++){
        if(weights[i]>0){
            vec4 fposition = vec4(position,1.0);
            fposition = fposition*transform_m[joint_mi[i]];
            fposition = fposition*weights[i];
            accPosition = accPosition + fposition;
        }else{
            break;
        }
    }
    accPosition[3]=1.0;
    mat4 modelview = view * model;
    gl_Position = perspective * modelview * accPosition;
    UV = texture;
}
pub const SHADER: &str = r#"
precision mediump float;

varying vec2 v_texturecoord;
varying vec3 v_normal;
//uniform sampler2D u_Sampler;
varying vec3 v_lighting;

void main(void) {
    vec4 texelColor = vec4(v_texturecoord.s, v_texturecoord.t, 0.5, 1.0); //texture2D(u_Sampler, vec2(v_texturecoord.s, v_texturecoord.t));
    gl_FragColor = vec4(texelColor.rgb * v_lighting, 1.0);
}"#;
#version 150

in vec2 v_uv;
in vec4 v_color;

uniform sampler2D u_texture;
uniform sampler2D u_overlay;
uniform float u_red;
uniform float u_green;
uniform float u_blue;

out vec4 o_color;

//vec4 resultCol;
//extern vec2 stepSize;

//vec4 effect( vec4 col, Image texture, vec2 v_uv, vec2 screenPos )
vec2 stepSize = vec2(0.030, 0.030);

void main()
{
  // get color of pixels:
  float alpha = 4*texture( u_texture, v_uv ).a;
  alpha -= texture( u_texture, v_uv + vec2( stepSize.x, 0.0 ) ).a;
  alpha -= texture( u_texture, v_uv + vec2( -stepSize.x, 0.0 ) ).a;
  alpha -= texture( u_texture, v_uv + vec2( 0.0, stepSize.y ) ).a;
  alpha -= texture( u_texture, v_uv + vec2( 0.0, -stepSize.y ) ).a;

  // calculate resulting color
  vec4 resultCol = vec4( 0.2, 0.8, 0.5, alpha );
  if (resultCol.a == 0) {
    o_color = texture(u_texture, v_uv);
  } else {
    o_color = resultCol;
  }
}


shader_type canvas_item;

uniform sampler2D my_texture;

vec4 color_lookup(uint pixel_type) {
    if (pixel_type == uint(0)) return vec4(0, 1, 1, 0.2);
    else if (pixel_type == uint(1)) return vec4(1, 1, 0, 1);
    else if (pixel_type == uint(2)) return vec4(0.6, 0.3, 0.3, 1);
    else if (pixel_type == uint(3)) return vec4(0.145, 0.317, 0.592 , 0.7);
    else return vec4(0.2, 0, 0.2, 1);
}

void fragment() {
    //uint pixel_type = uint(texture(my_texture, UV).r);
    uint pixel_type = uint(texture(my_texture, UV).r * 256.0 + 0.1);
    COLOR = color_lookup(pixel_type);
    //COLOR = vec4(texture(my_texture, UV).r, 1, 1, 1);
    //COLOR = vec4(1, 0, 1, 1);
}
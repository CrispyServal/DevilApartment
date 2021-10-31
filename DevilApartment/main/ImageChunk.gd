extends Sprite


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var image: Image
var image_texture: ImageTexture

# Called when the node enters the scene tree for the first time.
func _ready():
    image = Image.new()
    image.create(256, 256, false, Image.FORMAT_R8)
    
    image_texture = ImageTexture.new()
    image_texture.create(256, 256, Image.FORMAT_R8, ImageTexture.STORAGE_RAW)
    #debug()
    
    
func debug():
    var HalfChunk = load("res://main/fallingsand/HalfChunk.gd")
    var half_chunk: HalfChunk = HalfChunk.new()
    half_chunk.init(32, 0, 0)
    update_image(half_chunk, 0, 0)
    
func update_image(half_chunk, start_x, start_y):
    #print("update image for (%d, %d), start: (%d, %d)" % [half_chunk.row, half_chunk.col, start_x, start_y])
    image.lock()
    var buffer = half_chunk.get_buffer()
    for y in range(half_chunk.get_size()):
        for x in range(half_chunk.get_size()):
            var pixel: int = buffer[y][x] & 0xff
            #print("(%d, %d) -> %d" % [x, y, pixel])
            image.set_pixel(start_x + x, start_y + y, Color8(pixel, 0, 0))
    image.unlock()
    #image.save_png("res://debug_output/texture.png")
    image_texture.set_data(image)
    
    var mat: ShaderMaterial = material
    mat.set_shader_param("my_texture", image_texture)
    #set_texture(image_texture)

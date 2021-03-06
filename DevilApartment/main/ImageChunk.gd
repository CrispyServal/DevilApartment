extends Sprite


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var image: Image
var image_texture: ImageTexture
const TEXTURE_SIZE: int = Consts.TEXTURE_SIZE
const CHUNK_SIZE = Consts.CHUNK_SIZE

# Called when the node enters the scene tree for the first time.
func _ready():
    image = Image.new()
    image.create(TEXTURE_SIZE, TEXTURE_SIZE, false, Image.FORMAT_R8)
    
    image_texture = ImageTexture.new()
    image_texture.create(TEXTURE_SIZE, TEXTURE_SIZE, Image.FORMAT_R8, ImageTexture.STORAGE_RAW)
    
func update_image(pixel_world, start_x, start_y, offset_x, offset_y):
    #print("update image start: (%d, %d), offset: (%d, %d)" % [start_x, start_y, offset_x ,offset_y])
    image.lock()
    for y in range(offset_y, offset_y + CHUNK_SIZE):
        for x in range(offset_x, offset_x + CHUNK_SIZE):
            var id: int = pixel_world.get_pixel(start_x + x, start_y + y)
            #print("(%d, %d) -> %d" % [x, y, pixel])
            image.set_pixel(x, y, Color8(id, 0, 0))
    image.unlock()
    #image.save_png("res://debug_output/texture.png")
    image_texture.set_data(image)
    
    var mat: ShaderMaterial = material
    mat.set_shader_param("my_texture", image_texture)
    #set_texture(image_texture)
extends Sprite


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var image: Image
var image_texture: ImageTexture
const TEXTURE_SIZE: int = Consts.TEXTURE_SIZE
const CHUNK_SIZE = Consts.CHUNK_SIZE

# Called when the node enters the scene tree for the first time.
func _ready():
    image = Image.new()
    image.create(TEXTURE_SIZE, TEXTURE_SIZE, false, Image.FORMAT_R8)
    
    image_texture = ImageTexture.new()
    image_texture.create(TEXTURE_SIZE, TEXTURE_SIZE, Image.FORMAT_R8, ImageTexture.STORAGE_RAW)
    
func update_image(pixel_world, start_x, start_y, offset_x, offset_y):
    #print("update image start: (%d, %d), offset: (%d, %d)" % [start_x, start_y, offset_x ,offset_y])
    image.lock()
    for y in range(offset_y, offset_y + CHUNK_SIZE):
        for x in range(offset_x, offset_x + CHUNK_SIZE):
            var id: int = pixel_world.get_pixel(start_x + x, start_y + y)
            #print("(%d, %d) -> %d" % [x, y, pixel])
            image.set_pixel(x, y, Color8(id, 0, 0))
    image.unlock()
    #image.save_png("res://debug_output/texture.png")
    image_texture.set_data(image)
    
    var mat: ShaderMaterial = material
    mat.set_shader_param("my_texture", image_texture)
    #set_texture(image_texture)

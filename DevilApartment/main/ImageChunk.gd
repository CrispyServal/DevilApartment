extends Sprite


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var image
var image_texture

# Called when the node enters the scene tree for the first time.
func _ready():
    image = Image.new()
    image.create(256, 256, false, Image.FORMAT_R8)
    
    image_texture = ImageTexture.new()
    image_texture.create(256, 256, Image.FORMAT_R8, ImageTexture.STORAGE_RAW)
    update_image()
    

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
    #update_image()
    pass
    
func update_image():
    image.lock()
    for y in range(0, 256):
        for x in range(0, 256):
            var pixel: int = randi() % 3
            image.set_pixel(x, y, Color8(1, 0, 0))
    image.unlock()
    image_texture.set_data(image)
    
    material.set_shader_param("my_texture", image_texture)
    #set_texture(image_texture)

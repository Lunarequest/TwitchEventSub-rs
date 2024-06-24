extends Sprite2D


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func _on_twitch_event_chat_message(username, message):
	message = message.to_lower();

	if message.contains("godot"):
		var shader = (material as ShaderMaterial);
		var opacity = shader.get_shader_parameter("opacity");
		shader.set_shader_parameter("opacity", 1.0 - opacity);


extends Node2D
## Main scene controller - handles menu and game transitions

@onready var menu_container: Control = $MenuContainer
@onready var game_container: Node2D = $GameContainer
@onready var game_board = $GameContainer/GameBoard

var is_playing := false

func _ready() -> void:
	$MenuContainer/PlayButton.pressed.connect(_on_play_pressed)
	$MenuContainer/QuitButton.pressed.connect(_on_quit_pressed)

	# Connect to game board signals
	if game_board:
		game_board.score_changed.connect(_on_score_changed)
		game_board.match_found.connect(_on_match_found)
		game_board.rotation_triggered.connect(_on_rotation_triggered)

		# Connect back button
		var back_button = game_board.get_node("HUD/BackButton")
		if back_button:
			back_button.pressed.connect(_on_back_pressed)

	print("Revolving Match-3 (Rust Edition) - Ready")

func _on_play_pressed() -> void:
	print("Starting game...")
	is_playing = true
	menu_container.visible = false
	game_container.visible = true

	if game_board:
		game_board.reset()

func _on_quit_pressed() -> void:
	get_tree().quit()

func _on_back_pressed() -> void:
	print("Returning to menu...")
	is_playing = false
	game_container.visible = false
	menu_container.visible = true

func _on_score_changed(new_score: int) -> void:
	if game_board:
		var score_label = game_board.get_node("HUD/ScoreLabel")
		if score_label:
			score_label.text = "Score: %d" % new_score

func _on_match_found(count: int) -> void:
	print("Match found! Count: %d" % count)

func _on_rotation_triggered() -> void:
	print("Rotation triggered!")

func _process(delta: float) -> void:
	if is_playing and game_board:
		# Update rotation timer display
		var rotation_label = game_board.get_node("HUD/RotationLabel")
		if rotation_label:
			var interval = game_board.rotation_interval
			# The timer is internal to GameBoard, so we'll just show the interval
			rotation_label.text = "Rotation every: %.1fs" % interval

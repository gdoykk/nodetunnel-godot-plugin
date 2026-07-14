@tool
extends EditorPlugin

var update_check = preload("updater/update_check.gd").new()

func _enter_tree():
	# Update check disabled: it makes an outbound network request to
	# GitHub's API every time the editor loads this plugin. Uncomment
	# below to re-enable it.
	# add_child(update_check)
	# update_check.check_update(get_plugin_version())
	pass

func _exit_tree():
	pass

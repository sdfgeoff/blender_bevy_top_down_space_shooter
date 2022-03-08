.SUFFIXES:

BLENDER = blender


.PHONY: assets run fmt fmt-test ref-assets

assets:
	rm -r game/assets/scenes || true
	$(BLENDER) -b assets_src/Levels/Level1.blend --python ./scripts/export.py --python-exit-code=1  -- --output-file="game/assets/scenes/Level1.scn" --log-level=DEBUG
	$(BLENDER) -b assets_src/Particles/Particles.blend --python ./scripts/export.py --python-exit-code=1  -- --output-file="game/assets/scenes/Particles.scn" --log-level=DEBUG

run:
	cargo run game --release

fmt:
	cargo fmt
	python -m black blender_bevy_toolkit
	
fmt-test:
	# Format
	cargo fmt --all --check
	python -m black --diff --check blender_bevy_toolkit
	
	# Linting python requires the blender environment
	$(BLENDER) -b --python ./scripts/pylint_in_blender.py --python-exit-code=1
	
	# Dead code check (python)
	python -m vulture --min-confidence 100 blender_bevy_toolkit

test:
	$(BLENDER) -b --python ./scripts/pytest_in_blender.py --python-exit-code=1
	cargo test

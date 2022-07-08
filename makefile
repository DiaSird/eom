default: run

ch:
		cargo check

run:
		cargo run

test:
		python tests/msd_model.py

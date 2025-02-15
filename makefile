examples_dir=examples
tests_dir=tests

include tests/makefile
include examples/makefile

update-python-requirements:
	pipreqs ./ --force

vscode:
	scripts/vscode_prepublish.sh
	rsync -a extensions/husky-analyzer ~/.vscode/extensions/
	cargo install --path crates/apps/husky-analyzer --bin husky-analyzer-server

install-compiler:
	cargo install --path crates/apps/husky-compiler --bin husky-compiler

count-todo:
	scripts/pattern_statistics.py "todo!()" crates 1 10
	scripts/pattern_statistics.py "todo!()" crates 2 10

update-expect:
	UPDATE_EXPECT=1 cargo test --package husky-token -j 1 -- --nocapture
	UPDATE_EXPECT=1 cargo test --package husky-expr-syntax -j 1 -- --nocapture
	UPDATE_EXPECT=1 cargo test --package husky-infer -j 1 -- --nocapture

ubuntu-setup:
	scripts/ubuntu_setup.sh

test-digitize:
	cargo run --bin digitize -- data/typical-huskies0/n02109961_57.JPEG

test-digitize-ultraman:
	cargo run --bin digitize -- data/ultraman/leo/images.jpeg
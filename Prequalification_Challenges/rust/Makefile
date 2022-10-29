prog :=pmc

debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

build:
	cargo build $(release)
	#strip -s ./target/release/$(prog)
	#strip --strip-unneeded -R .note -R .comment -R .gnu.version -R .note.ABI-tag ./target/release/$(prog)
	cp ./src/flag.txt ./target/release/
	cp ./src/p.txt ./target/release/
	cp ./src/q.txt ./target/release/

help:
	@echo "usage: make $(prog) [debug=1]"

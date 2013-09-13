all: check
	rustpkg build escape_html

check:
	rust test src/escape_html/lib.rs

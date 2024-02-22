# This is a test Makefile
include included.mk

test: foo bar # This is a test target
	@echo "test target"

foo: # This is a foo target
	@echo "foo target"

bar: # This is a bar target
	@echo "bar target"

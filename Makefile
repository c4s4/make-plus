# This is a test Makefile
include included.mk

foo: bar # This is foo target
	@echo "foo target"

bar: # This is bar target
	@echo "bar target"

bug:
	@echo "bug target"

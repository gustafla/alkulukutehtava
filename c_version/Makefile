# basic configuration
EXECUTABLE:=alkuluku
PREFIX:=~/.local
CC:=gcc
STRIP:=strip --strip-all
CFLAGS:=-std=c99 -Wall -Wextra -Wpedantic

# library packages for pkg-config
PKGS:=

# build for debugging by default
DEBUG?=1

# don't use debug settings if building for the purpose of installing and use
# native arch settings for compilation
ifeq ($(MAKECMDGOALS),install)
DEBUG:=0
CFLAGS+=-march=native
endif

# debug and release build flags
ifeq ($(DEBUG),0)
BUILDDIR:=release
CFLAGS+=-O3
else
BUILDDIR:=debug
CFLAGS+=-Og -g -fsanitize=address -fsanitize=undefined -fno-omit-frame-pointer
endif

# add pkg-config libs and cflags
ifneq ($(strip $(PKGS)),)
CFLAGS+=$(shell pkg-config --cflags $(PKGS))
LDLIBS+=$(shell pkg-config --libs $(PKGS))
endif

SOURCEDIR:=src
TARGET:=$(BUILDDIR)/$(EXECUTABLE)
SOURCES:=$(wildcard $(SOURCEDIR)/*.c)
OBJS:=$(patsubst %.c,%.o,$(SOURCES:$(SOURCEDIR)/%=$(BUILDDIR)/%))

$(TARGET): $(OBJS)
	$(info Linking $@)
	@mkdir -p $(@D)
	@$(CC) -o $(TARGET) $(CFLAGS) $(OBJS) $(LDLIBS)
ifeq ($(DEBUG),0)
	$(info Stripping $@)
	@$(STRIP) $(TARGET)
endif

$(BUILDDIR)/%.o: $(SOURCEDIR)/%.c
	$(info Compiling $<)
	@mkdir -p $(@D)
	@$(CC) $(CFLAGS) -c -o $@ $<

.PHONY: clean install

clean:
	rm -rf $(BUILDDIR)

install: $(TARGET)
	cp $(TARGET) $(PREFIX)/bin


# Always run parallel jobs
MAKEFLAGS = -j

# Config
CC=g++
CC_FLAGS = -std=gnu++11 -Wall -Wextra -g
CC_FLAGS += $(pkg-config --cflags sfml-all)
CC_FLAGS += -Iinclude
# Link to .a and .so files in the lib/ directory, during linking AND at runtime
LD_FLAGS = -Llib -Wl,-rpath=lib
# Link liblua.a
LD_FLAGS += -llua -ldl -lm
# Link to SFML
LD_FLAGS += $(shell pkg-config --libs sfml-all)
# Link to STP (our TMX library)
LD_FLAGS += -lSTP -lpugixml
PLATFORM = linux # This is for building Lua and other libraries

# Files
EXECUTABLE = deucalion
SOURCES = $(wildcard src/*.cpp)
OBJECTS = $(SOURCES:.cpp=.o)

# All
all: $(EXECUTABLE)

# Primary target
$(EXECUTABLE): lib/liblua.a lib/libSTP.so lib/libpugixml.so lib $(OBJECTS)
	@$(CC) $(OBJECTS) -o $(EXECUTABLE) $(LD_FLAGS)
	@echo "Built $(EXECUTABLE)"

#Libraries
LUAVERSION = 5.3.2
lib/liblua.a:
	@cd lib/lua-$(LUAVERSION)/ && $(MAKE) -j --silent $(PLATFORM)
	@cp lib/lua-$(LUAVERSION)/src/liblua.a ./lib

# PugiXML is built at the same time as STP, so we can just grab it from there if that build succeeded
#	(and if it didn't, we lose, so who cares)
lib/libpugixml.so: lib/libSTP.so
	@cp lib/stp/build/lib/libpugixml.so ./lib

lib/libSTP.so:
	@cd lib/stp/ && mkdir -p build && cd build && cmake -q .. > /dev/null && make -j --silent
	@cp lib/stp/build/lib/libSTP.so ./lib

# Obtain object files
%.o: %.cpp
	@echo "Compiling $<..."
	@$(CC) -c $(CC_FLAGS) $< -o $@

# Remove generated object files and libraries
clean:
	@echo "Cleaning deucalion objects..."
	@rm -f $(EXECUTABLE) $(OBJECTS)
# Remove generated object files AND libraries
cleanall: clean
	@echo "Cleaning Lua $(LUAVERSION)..."	
	@cd lib/lua-5.3.2 && $(MAKE) --silent clean
	@echo "Cleaning STP..."
	@rm -rf lib/stp/build
	@echo "Cleaning .so and .a files in main directory..."
	@rm -f lib/*.so lib/*.a

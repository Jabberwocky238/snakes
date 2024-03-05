# 设置变量，指示编译器位置和目标文件名
CC = g++
CFLAGS = -Wall
EXECUTABLE = snake
# mingw32-make

# 设置源文件
SOURCES = main.cpp

# 编译规则
$(EXECUTABLE):$(SOURCES)
	$(CC) $(CFLAGS) $^ -o $@

# 默认任务
.PHONY: all clean
all: $(EXECUTABLE)
	@echo "Build complete."

# 运行程序
run: $(EXECUTABLE)
	./$(EXECUTABLE)

# 强制重新编译
.SECONDAry:
	$(MAKE)$(AM_MAKEFLAGS)

#include <cstdio>

extern "C" {
    int run(int argc, const char** argv) {
        printf("[C++] argc: %d\n", argc);
        for (int i = 0; i < argc; i++) {
            printf("[C++] argv[%d]: %s\n", i, argv[i]);
        }
        return 0;
    }
}

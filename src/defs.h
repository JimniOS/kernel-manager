#ifndef DEF_H
#define DEF_H

#define        true 1
#define        false 0
#define        ERR -1
#define        bool int

/**
 * @brief Log print
 *
 * @param fmt character string
 * @param ... N number of arguments to use with fmt.
 */

#define RED "\033[0;31m"
#define WHITE "\033[0;37m"
#define PANIC(...)                         \
        fprintf(stderr,"%sERROR%s: ", RED, WHITE); \
        printf(__VA_ARGS__);
#define LOG(...)         \
        printf("\nLOG: "); \
        printf(__VA_ARGS__);\
        printf("\n");

#endif

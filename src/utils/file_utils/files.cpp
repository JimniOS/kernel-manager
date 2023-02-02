#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <fcntl.h>
#include <unistd.h>
#include <fstream>
#include <signal.h>
#include <setjmp.h>
#include "../md5.h"
#include "../../defs.h"

#include "files.hpp"

#define CHUNK 32768

static void print_error_and_exit(const char *__restrict__ name, int code, 
                                 const char *__restrict__ error_msg);


static void create_file(const char *__restrict__ path);
static void *terminate_addr = NULL;

static void
terminate_intr(int signal)
{
        PANIC("\nCaught termination signal %d\n");
        _exit(signal);
}

/**
 * @brief Captures term signals sent to the program.
 * Redirects to terminate_intr
 *
 * @param term_addr Misc option. Defaults to *0x0
 */
static void capture_terminate(jmp_buf term_addr)
{
        terminate_addr = term_addr;
        signal(SIGHUP, terminate_intr);
        signal(SIGINT, terminate_intr);
        signal(SIGPIPE, terminate_intr);
        signal(SIGTERM, terminate_intr);
        signal(SIGUSR1, terminate_intr);
        signal(SIGUSR2, terminate_intr);
}
/**
 * @brief Return all capture handles to system.
 *
 */
static void uncapture_terminate(void)
{
        terminate_addr = NULL;
        signal(SIGHUP, SIG_DFL);
        signal(SIGINT, SIG_DFL);
        signal(SIGPIPE, SIG_DFL);
        signal(SIGTERM, SIG_DFL);
        signal(SIGUSR1, SIG_DFL);
        signal(SIGUSR2, SIG_DFL);
}

static void 
print_error_and_exit(
    const char *__restrict__ name,
    int code,
    const char *__restrict__ error_msg)
{
        PANIC(
            "Program panicked trying to access file.\n"
            "\tName: %s"
            "\n\tCode: %d"
            "\n\tErr:  %s\n",
            name, code, error_msg);
        exit(ERR);
}

static void 
create_file(const char *path)
{
        fclose(fopen(path, "w"));
}

extern "C" FILE *
access_file(const char *__restrict__ path, const char *__restrict__ permissions)
{
        int code = 0;
        if (strcmp(permissions, "r") == 0)
        {
                code = access(path, R_OK);
        }
        else if (strcmp(permissions, "w") == 0)
        {
                // we create the file when we have to write
                create_file(path);
                code = access(path, W_OK);
        }
        else if (strcmp(permissions, "f") == 0)
        {
                code = access(path, F_OK);
        }

        if (code == F_OK)
        {
                return fopen(path, permissions);
        }
        else
        {
                print_error_and_exit(path, code, "File doesnt exist or cant be accessed with current permissions");
        }
        return NULL;
}

extern "C" void 
copy_binaries(const char *src, const char *dest)
{
        printf("Copying file %s to %s... ", src, dest);
        std::string buffer;
        // Reads buffer directly from disk
        std::ifstream source_file(src);

        // read the entire buffer into string
        if (source_file.is_open())
        {

                source_file.seekg(0, std::ios::end);
                buffer.reserve(source_file.tellg());
                source_file.seekg(0, std::ios::beg);

                buffer.assign((std::istreambuf_iterator<char>(source_file)),
                              std::istreambuf_iterator<char>());

                source_file.close();
        }

        std::ofstream dest_file;
        dest_file.open(dest);
        if (dest_file.is_open())
        {
                dest_file << buffer;
        }
        printf("Done!\n");
        dest_file.close();
}

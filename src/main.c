#ifdef __cplusplus
#pragma warning("Not recomended to compile the file with a C++ compiler. Consider using a C compiler");
#endif

#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <signal.h>
#include <zlib.h>
#include <dirent.h>
#include <unistd.h>
#include <errno.h>
#include <setjmp.h>
#include "main.h"
#include "defs.h"
#include "utils/file_utils/files.hpp"
#include "utils/file_utils/gz_config.h"
#include "utils/md5.h"
#include <assert.h>
#include <getopt.h>

static bool _verbose;
static const char *custom_config_path = NULL;
static const char *custom_output_path = NULL;
static const char *custom_kernel_path = NULL;

int main(int argc, char **argv)
{
        PANIC("Still WIP");
}

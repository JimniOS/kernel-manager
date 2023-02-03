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
#include "get_patches.hpp"

static const char *custom_config_path = NULL;
static const char *custom_output_path = NULL;
int main(int argc, char **argv)
{
        int c;
        // get options
        while (1)
        {
                static struct option long_options[] =
                    {
                        {"config", required_argument, 0, 'c'},
                        {"output", required_argument, 0, 'o'},
                        {0, 0, 0, 0}};
                /* getopt_long stores the option index here. */
                int option_index = 0;

                c = getopt_long(argc, argv, "c:o:",
                                long_options, &option_index);

                /* Detect the end of the options. */
                if (c == -1)
                        break;

                switch (c)
                {
                case 0:
                        /* If this option set a flag, do nothing else now. */
                        if (long_options[option_index].flag != 0)
                                break;
                        printf("option %s", long_options[option_index].name);
                        if (optarg)
                                printf(" with arg %s", optarg);
                        printf("\n");
                        break;

                case 'c':
                        custom_config_path = optarg;
                        break;

                case 'o':
                        custom_output_path = optarg;
                        break;
                case '?':
                        /* getopt_long already printed an error message. */
                        break;
                }
        }
        
}

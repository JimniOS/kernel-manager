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
#include "get_patches.h"

static const char *input_path = NULL;
static const char *config_output_path = NULL;
static const char *custom_kernel_path = NULL;
static bool _extraction = false;
static bool _dist_kernel = false;

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
                        {"extract", no_argument, &_extraction, 1},
                        {"dist", no_argument, &_dist_kernel, 1},
                        {"kernel-directory", required_argument, 0, 'k'},

                        {0, 0, 0, 0}};
                /* getopt_long stores the option index here. */
                int option_index = 0;

                c = getopt_long(argc, argv, "c:o:ed",
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
                        input_path = optarg;
                        break;

                case 'o':
                        config_output_path = optarg;
                        break;
                case '?':
                        /* getopt_long already printed an error message. */
                        break;
                case 'e':
                        _extraction++;
                        break;
                case 'd':
                        _dist_kernel++;
                        break;
                case 'k':
                        custom_kernel_path = optarg;
                        break;
                }
        }

        if (custom_kernel_path != NULL && config_output_path != NULL)
        {
                PANIC("Cannot specify custom kernel directory AND custom output path.\n"
                      "Custom output path will be set according to the kernel path. Aborting");
                exit(1);
        }
        else if (custom_kernel_path != NULL && config_output_path == NULL)
        {
                custom_kernel_path = malloc((sizeof(custom_kernel_path) + sizeof(".config")) * sizeof(char));
                sprintf(config_output_path, "%s/.config", custom_kernel_path);
                LOG("CONFIG: %s", config_output_path);
        }
        else if (config_output_path != NULL && custom_kernel_path == NULL)
        {
                custom_kernel_path = malloc(sizeof(config_output_path));
                custom_kernel_path = config_output_path;
                // give better  way to do this; size of /.config = 8
                // get kernel build directory from config output path

                for (int i = 0; i < strlen(config_output_path) - 8; i++)
                {
                        PANIC("UNIMPLEMENTED");
                }
                LOG("KERNEL: %s", custom_kernel_path);
        }

        if (_dist_kernel)
        {
                // we will ask and download the choice in this case
                // there are no overrides to be written here
        }

        if (_extraction)
        {
                if (input_path == NULL)
                {
                        input_path = "/proc/config.gz";
                }
                if (config_output_path == NULL)
                {
                        config_output_path = "kernel/.config";
                }
                get_config(access_file(input_path, "r"), access_file(config_output_path, "w"));
        }
        else
        {
                copy_binaries(input_path,config_output_path);
        }
}

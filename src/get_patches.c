#include "get_patches.h"
#include <stdlib.h>
#include <stdio.h>
#include "defs.h"
#include <curl/curl.h>
#include "utils/file_utils/files.hpp"

// download file from url to file
// return 0 on success
int 
get_patches()
{
        const char *url = "https://files.testfile.org/ZIPC/15MB-Corrupt-Testfile.Org.zip";
        const char *file = "patches/test_patches_file.zip";
        printf("Downloading %s to %s \n", url, file);
        CURL *curl;
        FILE *fp;
        CURLcode res;
        curl = curl_easy_init();
        if (curl)
        {
                fp = access_file(file, "wb");
                curl_easy_setopt(curl, CURLOPT_URL, url);
                curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, NULL);
                curl_easy_setopt(curl, CURLOPT_WRITEDATA, fp);
                res = curl_easy_perform(curl);
                if (res != CURLE_OK)
                {
                        PANIC("Curling the file failed. Curl returned a non-ok status\n");
                }
                curl_easy_cleanup(curl);
                fclose(fp);
        }
        else
        {
                PANIC("Curling the file failed\n");
                exit(1);
        }

        return 0;
}

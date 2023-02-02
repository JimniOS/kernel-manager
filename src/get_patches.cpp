#include "get_patches.hpp"
#include <iostream>
#include "defs.h"
#include <curl/curl.h>

// download file from url to file
// return 0 on success
extern "C" void get_patches()
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
        fp = fopen(file, "wb");
        curl_easy_setopt(curl, CURLOPT_URL, url);
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, NULL);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, fp);
        res = curl_easy_perform(curl);
        curl_easy_cleanup(curl);
        fclose(fp);
    }
}

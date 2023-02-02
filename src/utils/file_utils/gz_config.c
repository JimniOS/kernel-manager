#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <fcntl.h>
#include <unistd.h>
#include "../../defs.h"
#include "gz_config.h"
#include <zlib.h>
#include <signal.h>
#include <signal.h>
#include <setjmp.h>
#define CHUNK 32768
static ssize_t processed = 0;
static void *terminate_addr = NULL;
/**
 * @brief Hard exits process with passed signal.
 *
 * @param signal signal to exit with
 */
static void
terminate_intr(int signal)
{
        PANIC("\nCaught termination signal %d\n Processed blocks %zd", signal, processed);
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

int get_config(FILE *gz_file, FILE *output_file)
{
        capture_terminate(NULL);
        int ret;
        unsigned have;
        z_stream strm;
        unsigned char in[CHUNK];
        unsigned char out[CHUNK];

        /* allocate inflate state */
        strm.zalloc = Z_NULL;
        strm.zfree = Z_NULL;
        strm.opaque = Z_NULL;
        strm.avail_in = 0;
        strm.next_in = Z_NULL;
        ret = inflateInit2(&strm,MAX_WBITS+16);
        if (ret != Z_OK)
        {
                fclose(gz_file);
                return ret;
        }

        /* decompress until deflate stream ends or end of file */
        do
        {
                strm.avail_in = fread(in, 1, CHUNK, gz_file);
                if (ferror(gz_file))
                {
                        (void)inflateEnd(&strm);
                        fclose(gz_file);
                        return Z_ERRNO;
                }
                if (strm.avail_in == 0)
                        break;
                strm.next_in = in;

                /* run inflate() on input until output buffer not full */
                do
                {
                        strm.avail_out = CHUNK;
                        strm.next_out = out;
                        ret = inflate(&strm, Z_NO_FLUSH);
                        size_t got = CHUNK - strm.avail_out;
                        processed+=got;
                        assert(ret != Z_STREAM_ERROR); /* state not clobbered */
                        switch (ret)
                        {
                        case Z_NEED_DICT:
                                fprintf(stderr, "Need Dict error - ");
                                ret = Z_DATA_ERROR; /* and fall through */
                        case Z_DATA_ERROR:
                                (void)inflateEnd(&strm);
                                fclose(gz_file);
                                fprintf(stderr, "Data Error\n");
                                return ret;
                        case Z_MEM_ERROR:
                                (void)inflateEnd(&strm);
                                fclose(gz_file);
                                fprintf(stderr, "Out of memory\n");
                                return ret;
                        }
                        have = CHUNK - strm.avail_out;
                        if (fwrite(out, 1, have, output_file) != have || ferror(stdout))
                        {
                                (void)inflateEnd(&strm);
                                fclose(gz_file);
                                PANIC("Error writing stream\n");
                                return Z_ERRNO;
                        }
                } while (strm.avail_out == 0);

                /* done when inflate() says it's done */
        } while (ret != Z_STREAM_END);

        /* clean up and return */
        (void)inflateEnd(&strm);
        fclose(gz_file);
        return ret == Z_STREAM_END ? Z_OK : Z_DATA_ERROR;
        uncapture_terminate();
}
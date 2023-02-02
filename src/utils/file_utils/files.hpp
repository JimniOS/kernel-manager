#ifdef __cplusplus
extern "C"
{
#endif
        void copy_binaries(const char *src, const char *dest);
        FILE *access_file(const char *__restrict__ path, const char *__restrict__ permissions);
#ifdef __cplusplus
}
#endif

project('KernelManager', ['c','cpp'])


add_project_arguments('-Wno-unused-result',language: 'c') # Stop stupid warnings
add_project_arguments('-Werror=implicit-function-declaration',language: ['c','cpp']) # Panic compiler if a undeclared function is used

libz = dependency('zlib')

sources = files([
    'src/main.c',
    'src/main.h',
])

file_helper = library('file_helper', sources : ['src/utils/file_utils/files.cpp', 'src/utils/file_utils/files.hpp'])
zlib_helper = library('zlib_helper', sources : ['src/utils/file_utils/gz_config.c', 'src/utils/file_utils/gz_config.h'],dependencies : libz)
md5_lib = library('md5_lib' , sources: ['src/utils/md5.c','src/utils/md5.h'])

hello_lib = library('hello_lib', sources: ['src/get_patches.cpp','src/get_patches.hpp'],dependencies: dependency('libcurl'))
executable('kernel-manager', 
        sources,dependencies : libz , 
        c_args:['-fno-stack-protector', 
                '-Wno-unused-result',
                '-Wno-uninitialized',
                ],
        link_with: [zlib_helper,file_helper,md5_lib,hello_lib],
        )
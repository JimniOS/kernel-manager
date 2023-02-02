project('KernelManager', ['c','cpp'])

if get_option('buildtype') == 'release'
  add_project_arguments('-O3', '-flto',  language : 'c')
endif

if get_option('buildtype') == 'debug'
  add_project_arguments('-Og', '-g3',  language : 'c')
endif

add_project_arguments('-Wno-unused-result',language: 'c')

libz = dependency('zlib')

sources = files([
    'src/main.c',
    'src/main.h',
])

file_helper = library('file_helper', sources : ['src/utils/file_utils/files.cpp', 'src/utils/file_utils/files.hpp'])
zlib_helper = library('zlib_helper', sources : ['src/utils/file_utils/gz_config.c', 'src/utils/file_utils/gz_config.h'],dependencies : libz)
md5_lib = library('md5_lib' , sources: ['src/utils/md5.c','src/utils/md5.h'],c_args:['-Werror=implicit-function-declaration'])

executable('kernel-manager', 
        sources,dependencies : libz , 
        c_args:['-fno-stack-protector', 
                '-Wno-unused-result',
                '-Wno-uninitialized',
                ],
        link_with: [zlib_helper,file_helper,md5_lib],
        )
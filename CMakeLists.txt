cmake_minimum_required(VERSION 4.3.2)

set(CMAKE_EXPERIMENTAL_CXX_IMPORT_STD
    "f35a9ac6-8463-4d38-8eec-5d6008153e7d")

project(ewt LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(ewt SHARED src/lib.cc)
target_sources(ewt PRIVATE src/lib.def)

if(MSVC)
    target_compile_options(ewt PRIVATE /W4)
    target_compile_options(ewt PRIVATE /GR- /EHs-c-)
    target_compile_options(ewt PRIVATE /guard:cf)
    target_link_options(ewt PRIVATE /guard:cf)
    target_link_libraries(ewt runtimeobject)
else()
    target_compile_options(ewt PRIVATE -Wall -Wextra)
    target_compile_options(ewt PRIVATE -fno-rtti -fno-exceptions)
    target_compile_options(ewt PRIVATE -mguard=cf)
    target_link_options(ewt PRIVATE -mguard=cf)
    target_link_libraries(ewt -lruntimeobject)
endif()

# set_target_properties(ewt PROPERTIES CXX_MODULE_STD ON)
#
# file(GLOB_RECURSE IXX_FILES "C:/msys64/clangarm64/include/winrt/winrt_*.ixx")
#
# target_sources(ewt
#     PUBLIC
#         FILE_SET CXX_MODULES
#         BASE_DIRS "C:/msys64/clangarm64/include/winrt"
#         FILES ${IXX_FILES}
# )
#
# target_sources(ewt
#     PUBLIC
#         FILE_SET CXX_MODULES
#         BASE_DIRS "C:/msys64/clangarm64/include/winrt"
#         FILES "C:/msys64/clangarm64/include/winrt/winrt.Windows.Foundation.ixx" "C:/msys64/clangarm64/include/winrt/winrt.Windows.Foundation.Collections.ixx"
# )

set_target_properties(ewt PROPERTIES
    VERSION 0.1.0
    SOVERSION 1
    OUTPUT_NAME ewt
)

cmake_minimum_required(VERSION 4.3.2)

set(CMAKE_EXPERIMENTAL_CXX_IMPORT_STD
        "f35a9ac6-8463-4d38-8eec-5d6008153e7d")

project(ewt LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(ewt SHARED src/lib.cc)
target_sources(ewt PRIVATE src/lib.def)

set(EWT_BACKEND "ICU" CACHE STRING "Select the tokenize backend")
set_property(CACHE EWT_BACKEND PROPERTY STRINGS "WINRT" "ICU")
get_property(ALLOWED_BACKENDS CACHE EWT_BACKEND PROPERTY STRINGS)
if (NOT EWT_BACKEND IN_LIST ALLOWED_BACKENDS)
    message(FATAL_ERROR "Invalid BACKEND: `${EWT_BACKEND}`. Must be one of WINRT or ICU")
endif ()

if (EWT_BACKEND STREQUAL "WINRT")
    add_compile_definitions(USE_WINRT)
elseif (EWT_BACKEND STREQUAL "ICU")
    add_compile_definitions(USE_ICU)
endif ()

if (MSVC)
    target_compile_options(ewt PRIVATE /W4)
    target_compile_options(ewt PRIVATE /GR- /EHs-c-)
    target_compile_options(ewt PRIVATE /guard:cf)
    target_link_options(ewt PRIVATE /guard:cf)
    if (EWT_BACKEND STREQUAL "WINRT")
        target_link_libraries(ewt runtimeobject)
    endif ()
else ()
    target_compile_options(ewt PRIVATE -Wall -Wextra)
    target_compile_options(ewt PRIVATE -fno-rtti -fno-exceptions)
    target_compile_options(ewt PRIVATE -mguard=cf)
    target_link_options(ewt PRIVATE -mguard=cf)
    set(CMAKE_SHARED_LINKER_FLAGS_RELEASE "${CMAKE_SHARED_LINKER_FLAGS_RELEASE} -s")
    if (EWT_BACKEND STREQUAL "WINRT")
        target_link_libraries(ewt -lruntimeobject)
    elseif (EWT_BACKEND STREQUAL "ICU")
        target_link_libraries(ewt -licudt -licuuc)
    endif ()
endif ()

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

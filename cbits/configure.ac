#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_INIT(thread, 1, https://github.com/uuhan/terminate-thread)
AC_CONFIG_SRCDIR([Thread.h])
AC_CONFIG_HEADERS([config.h])
AC_CONFIG_MACRO_DIR(m4)
AH_TOP([
#pragma once
])

AM_INIT_AUTOMAKE
AM_PROG_LIBTOOL

# Checks for programs.
AC_PROG_CXX
AC_PROG_CC

# Checks for libraries.
ACX_PTHREAD

# Checks for header files.
AC_CHECK_HEADERS()

# Checks for typedefs, structures, and compiler characteristics.
AC_CHECK_HEADER_STDBOOL
AC_C_INLINE

# Checks for library functions.
AC_CHECK_FUNCS([select])

AC_CONFIG_FILES([Makefile])
AC_OUTPUT

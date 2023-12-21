@echo off
gcc -c -o crand.o crand.c
ar rcs crand.lib crand.o
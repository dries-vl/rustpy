@echo off
set "PYTHONHOME=%~dp0python-3.11.5-embed"
set "PATH=%~dp0python-3.11.5-embed;%PATH%"
%~dp0target/debug/rustpy.exe
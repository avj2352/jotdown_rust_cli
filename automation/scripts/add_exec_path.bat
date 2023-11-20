@echo off

rem ..Change folder to release
pushd %~dp0\..\..\release

rem ..Check if the executable file exists
if not exist "%~1" (
    echo Error: The executable file '%~1' does not exist.
    exit /b 1
)

rem ..Check if the executable file is already in the PATH
if exist "%PATH%;%~1%" (
    echo The executable file '%~1' is already in the PATH.
    exit /b 0
)

rem ..Add the executable file to the PATH
set PATH=%~1%;%PATH%

rem ..Export the PATH variable
setx PATH "%PATH%"

rem ..Print the PATH variable
echo The PATH variable is now: %PATH%
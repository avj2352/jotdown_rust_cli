@echo off

rem ..Initialize
@echo Initializing clean build..
cd ../../
rmdir /S release

rem ..Create build
cargo build --release
@echo Created Jotdown release build!
@echo Creating release folder..
md release

rem ..Package
@echo Packaging executable and release information..
copy ..\..\target\release\jotdown.exe ..\..\release\jotdown.exe

rem ..Rename jotdown.exe to jd.exe
@echo Rename file - jd.exe
ren ..\..\release\jotdown.exe jd.exe

rem ..Copy release information
copy ../../*.md release/

rem ..done!
@echo Packaging completed !!
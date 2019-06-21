@ECHO OFF

SET APPID=com.mi.test

cargo build --target i686-pc-windows-msvc

mkdir dist\%APPID%

copy %~dp0\target\i686-pc-windows-msvc\debug\app.dll %~dp0\dist\%APPID%\app.dll
copy %~dp0\%APPID%.json %~dp0\dist\%APPID%\app.json

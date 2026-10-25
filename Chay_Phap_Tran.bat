@echo off
title Phap Tran Tu Tien - Desktop Overlay
chcp 65001 >nul
echo Dang khoi dong Phap Tran Tu Tien...
if exist "%~dp0bin\tu_tien_tran.exe" (
    start "" "%~dp0bin\tu_tien_tran.exe"
) else (
    start "" "%~dp0tu_tien_tran\target\release\tu_tien_tran.exe"
)
exit

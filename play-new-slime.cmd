@echo off
rem Larder World with the "new slime": the switches built 11-17 that have no measured harm, on together
rem (far smell, charges round walls; ROADMAP "Playtest pack 11-17"). Otherwise as play.cmd.
rem One line on purpose: cmd reads a batch file as it runs, and git pull may rewrite this file.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0play.ps1" far_smell=1 set=unit_paths=1 %* || pause

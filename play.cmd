@echo off
rem One-click Larder World: double-click to update to main, build, and open the start menu.
rem Extra arguments go to the game, e.g.  play.cmd map=nest mode=squads
rem One line on purpose: cmd reads a batch file as it runs, and git pull may rewrite this file.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0play.ps1" %* || pause

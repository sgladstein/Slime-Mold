@echo off
rem The slime viewer: double-click to update, build and open the game on each slime with a panel to spawn its events and tune its looks and rules (saved to slime-tweaks.txt, which the game then plays).
rem One line on purpose, as play.cmd: git pull may rewrite this file while it runs.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0play.ps1" viewer=1 %* || pause

net stop RusticLight
sc.exe delete RusticLight 
sc.exe create RusticLight binpath=%~dp0target\debug\rustic_light.exe type=own start=auto
net start RusticLight
pause
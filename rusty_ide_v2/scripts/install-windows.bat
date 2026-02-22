@echo off
setlocal enabledelayedexpansion

:: Rusty IDE v2 - Windows Installation Script
:: ===========================================

echo.
echo ============================================
echo   Rusty IDE v2 - Installation Script
echo ============================================
echo.

:: Check for admin rights
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo [ERROR] This installer requires administrator privileges.
    echo Please right-click and select "Run as administrator"
    pause
    exit /b 1
)

:: PHASE 1: CLEANUP
:: ================
echo [PHASE 1/5] Cleaning up old agent installation...
echo.

:: Remove old agent binary
if exist "%USERPROFILE%\.local\bin\agent.exe" (
    echo [*] Found old agent binary at %USERPROFILE%\.local\bin\agent.exe
    del /f /q "%USERPROFILE%\.local\bin\agent.exe" 2>nul
    if exist "%USERPROFILE%\.local\bin\agent.exe" (
        echo [WARNING] Could not delete old agent binary
    ) else (
        echo [OK] Deleted old agent binary
    )
)

:: Check for old agent startup scripts
if exist "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\agent.bat" (
    echo [*] Removing old agent startup script...
    del /f /q "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\agent.bat" 2>nul
    echo [OK] Removed old startup script
)

:: Ask about old agent data
if exist "%USERPROFILE%\.agent\" (
    echo.
    echo [?] Old agent data directory found at %USERPROFILE%\.agent\
    set /p DELETE_OLD_DATA="    Do you want to delete it? (y/N): "
    if /i "!DELETE_OLD_DATA!"=="y" (
        rmdir /s /q "%USERPROFILE%\.agent" 2>nul
        echo [OK] Deleted old agent data
    ) else (
        echo [OK] Kept old agent data
    )
)

echo.
echo [OK] Cleanup complete!
echo.

:: PHASE 2: BUILD RUSTY IDE
:: ========================
echo [PHASE 2/5] Building Rusty IDE in release mode...
echo.

:: Get the script directory
set SCRIPT_DIR=%~dp0
set PROJECT_DIR=%SCRIPT_DIR%..
set TAURI_DIR=%PROJECT_DIR%\src-tauri

if not exist "%TAURI_DIR%" (
    echo [ERROR] Tauri directory not found at %TAURI_DIR%
    pause
    exit /b 1
)

echo [*] Building from: %TAURI_DIR%
cd /d "%TAURI_DIR%"

echo [*] Running: cargo build --release
echo     This may take several minutes...
echo.

cargo build --release
if %errorLevel% neq 0 (
    echo.
    echo [ERROR] Build failed! Please check the errors above.
    pause
    exit /b 1
)

echo.
echo [OK] Build complete!
echo.

:: PHASE 3: INSTALL
:: ================
echo [PHASE 3/5] Installing Rusty IDE...
echo.

:: Create installation directory
set INSTALL_DIR=C:\Program Files\Rusty
if not exist "%INSTALL_DIR%" (
    echo [*] Creating installation directory: %INSTALL_DIR%
    mkdir "%INSTALL_DIR%"
)

:: Find the built binary
set BINARY_PATH=%TAURI_DIR%\target\release\rusty_ide_v2.exe
if not exist "%BINARY_PATH%" (
    echo [ERROR] Built binary not found at %BINARY_PATH%
    pause
    exit /b 1
)

:: Copy binary
echo [*] Copying rusty.exe to %INSTALL_DIR%
copy /y "%BINARY_PATH%" "%INSTALL_DIR%\rusty.exe" >nul
if %errorLevel% neq 0 (
    echo [ERROR] Failed to copy binary to installation directory
    pause
    exit /b 1
)

echo [OK] Binary installed to %INSTALL_DIR%\rusty.exe
echo.

:: Add to PATH if not already present
echo [*] Checking system PATH...
set PATH_TO_ADD=%INSTALL_DIR%
echo %PATH% | findstr /i /c:"%PATH_TO_ADD%" >nul
if %errorLevel% neq 0 (
    echo [*] Adding to system PATH...
    setx PATH "%PATH%;%PATH_TO_ADD%" /M >nul 2>&1
    if %errorLevel% equ 0 (
        echo [OK] Added to system PATH
        set "PATH=%PATH%;%PATH_TO_ADD%"
    ) else (
        echo [WARNING] Could not add to system PATH automatically
        echo           Please add manually: %PATH_TO_ADD%
    )
) else (
    echo [OK] Already in system PATH
)

echo.
echo [OK] Installation complete!
echo.

:: PHASE 4: SETUP DATA DIRECTORY
:: =============================
echo [PHASE 4/5] Setting up data directories...
echo.

set DATA_DIR=%USERPROFILE%\.rusty

echo [*] Creating directory structure at %DATA_DIR%
mkdir "%DATA_DIR%" 2>nul
mkdir "%DATA_DIR%\agent" 2>nul
mkdir "%DATA_DIR%\workspaces" 2>nul

:: Create default permissions.json
echo [*] Creating default permissions.json
echo { > "%DATA_DIR%\permissions.json"
echo   "version": "1.0", >> "%DATA_DIR%\permissions.json"
echo   "permissions": {} >> "%DATA_DIR%\permissions.json"
echo } >> "%DATA_DIR%\permissions.json"

echo [OK] Data directories created:
echo     - %DATA_DIR%\agent\
echo     - %DATA_DIR%\workspaces\
echo     - %DATA_DIR%\permissions.json
echo.

:: PHASE 5: VERIFY
:: ===============
echo [PHASE 5/5] Verifying installation...
echo.

:: Test the rusty command
"%INSTALL_DIR%\rusty.exe" --version >nul 2>&1
if %errorLevel% equ 0 (
    echo [OK] Rusty IDE is working correctly!
) else (
    echo [WARNING] Could not verify rusty command
)

echo.
echo ============================================
echo   Installation Complete!
echo ============================================
echo.
echo Rusty IDE v2 has been installed successfully!
echo.
echo Installation Location: %INSTALL_DIR%\rusty.exe
echo Data Directory: %DATA_DIR%
echo.
echo Usage:
echo   rusty [folder]     - Open folder in Rusty IDE
echo   rusty .            - Open current directory
echo   rusty --help       - Show help
echo.
echo NOTE: You may need to restart your terminal for PATH changes to take effect.
echo.
echo Creating desktop shortcut...
set DESKTOP=%USERPROFILE%\Desktop
powershell -Command "$WS = New-Object -ComObject WScript.Shell; $SC = $WS.CreateShortcut('%DESKTOP%\Rusty IDE.lnk'); $SC.TargetPath = '%INSTALL_DIR%\rusty.exe'; $SC.IconLocation = '%INSTALL_DIR%\rusty.exe,0'; $SC.Description = 'Rusty IDE v2'; $SC.Save()" 2>nul
if %errorLevel% equ 0 (
    echo [OK] Desktop shortcut created
) else (
    echo [INFO] Desktop shortcut creation skipped
)
echo.
echo Thank you for installing Rusty IDE v2!
echo.

pause

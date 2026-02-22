@echo off
setlocal enabledelayedexpansion

:: ============================================================================
:: Rusty IDE Installation Script for Windows
:: ============================================================================

echo.
echo ========================================
echo  Rusty IDE Installer for Windows
echo ========================================
echo.

:: Check for administrator privileges
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo [ERROR] This script requires administrator privileges.
    echo Please right-click and select "Run as administrator"
    pause
    exit /b 1
)

:: ============================================================================
:: CLEANUP PHASE
:: ============================================================================

echo [1/4] CLEANUP PHASE
echo ----------------------------------------

:: Check for old agent installation
set "OLD_AGENT=%USERPROFILE%\.local\bin\agent.exe"
if exist "%OLD_AGENT%" (
    echo [INFO] Found old agent installation at %OLD_AGENT%
    del /f /q "%OLD_AGENT%" 2>nul
    if exist "%OLD_AGENT%" (
        echo [WARNING] Could not delete old agent binary
    ) else (
        echo [SUCCESS] Removed old agent binary
    )
)

:: Remove old agent data directory
set "OLD_DATA=%USERPROFILE%\.agent"
if exist "%OLD_DATA%" (
    echo [INFO] Found old agent data directory at %OLD_DATA%
    set /p CONFIRM="Do you want to remove old agent data? (y/N): "
    if /i "!CONFIRM!"=="y" (
        rmdir /s /q "%OLD_DATA%" 2>nul
        if exist "%OLD_DATA%" (
            echo [WARNING] Could not delete old agent data
        ) else (
            echo [SUCCESS] Removed old agent data directory
        )
    ) else (
        echo [INFO] Keeping old agent data
    )
)

:: Clean up old PATH entries
echo [INFO] Cleaning up old PATH entries...
:: This is complex in batch, so we'll just note it
echo [INFO] Please manually remove old agent PATH entries if needed

echo.

:: ============================================================================
:: INSTALLATION PHASE
:: ============================================================================

echo [2/4] INSTALLATION PHASE
echo ----------------------------------------

:: Get the script's directory (where the project root should be)
set "SCRIPT_DIR=%~dp0"
:: Point to Rusty IDE directory
set "PROJECT_DIR=\workspace\jashan\rust_agent\rusty_ide_v2"

:: Check if Cargo.toml exists
if not exist "%PROJECT_DIR%\src-tauri\Cargo.toml" (
    echo [ERROR] Rusty IDE not found at %PROJECT_DIR%\src-tauri\
    pause
    exit /b 1
)

:: Build in release mode
echo [INFO] Building Rusty IDE in release mode...
cd /d "%PROJECT_DIR%\src-tauri"
cargo build --release
if %errorLevel% neq 0 (
    echo [ERROR] Build failed. Please check the error messages above.
    pause
    exit /b 1
)
echo [SUCCESS] Build completed successfully

:: Create installation directory
set "INSTALL_DIR=C:\Program Files\Rusty"
if not exist "%INSTALL_DIR%" (
    mkdir "%INSTALL_DIR%"
    if %errorLevel% neq 0 (
        echo [ERROR] Failed to create installation directory
        pause
        exit /b 1
    )
)

:: Copy binary
echo [INFO] Installing Rusty IDE to %INSTALL_DIR%...
copy /y "%PROJECT_DIR%\src-tauri\target\release\rusty-tui.exe" "%INSTALL_DIR%\rusty.exe"
if %errorLevel% neq 0 (
    echo [ERROR] Failed to copy binary
    pause
    exit /b 1
)
echo [SUCCESS] Binary installed successfully

:: Add to PATH if not already present
echo [INFO] Checking PATH configuration...
echo %PATH% | find /i "%INSTALL_DIR%" >nul
if %errorLevel% neq 0 (
    echo [INFO] Adding Rusty IDE to system PATH...
    setx /M PATH "%PATH%;%INSTALL_DIR%"
    if %errorLevel% neq 0 (
        echo [WARNING] Failed to update PATH automatically
        echo Please add %INSTALL_DIR% to your PATH manually
    ) else (
        echo [SUCCESS] Added to system PATH
        echo [INFO] Please restart your terminal for PATH changes to take effect
    )
) else (
    echo [INFO] Already in PATH
)

:: Create data directory
set "DATA_DIR=%USERPROFILE%\.rusty"
if not exist "%DATA_DIR%" (
    mkdir "%DATA_DIR%"
    echo [SUCCESS] Created data directory at %DATA_DIR%
)

:: Create desktop shortcut (optional)
set /p CREATE_SHORTCUT="Create desktop shortcut? (y/N): "
if /i "!CREATE_SHORTCUT!"=="y" (
    set "SHORTCUT=%USERPROFILE%\Desktop\Rusty IDE.lnk"
    powershell -Command "$WshShell = New-Object -ComObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%SHORTCUT%'); $Shortcut.TargetPath = '%INSTALL_DIR%\rusty.exe'; $Shortcut.WorkingDirectory = '%USERPROFILE%'; $Shortcut.Save()"
    echo [SUCCESS] Desktop shortcut created
)

echo.

:: ============================================================================
:: VERIFICATION PHASE
:: ============================================================================

echo [3/4] VERIFICATION PHASE
echo ----------------------------------------

:: Test the installation
echo [INFO] Testing Rusty IDE installation...
"%INSTALL_DIR%\rusty.exe" --version 2>nul
if %errorLevel% neq 0 (
    echo [WARNING] Command test failed. You may need to restart your terminal.
) else (
    echo [SUCCESS] Rusty IDE is working correctly
)

echo.

:: ============================================================================
:: SUCCESS MESSAGE
:: ============================================================================

echo [4/4] INSTALLATION COMPLETE
echo ========================================
echo.
echo  Rusty IDE has been successfully installed!
echo.
echo  Installation location: %INSTALL_DIR%\rusty.exe
echo  Data directory: %DATA_DIR%
echo.
echo ========================================
echo  USAGE INSTRUCTIONS
echo ========================================
echo.
echo  To start Rusty IDE:
echo    rusty
echo.
echo  To start in interactive mode:
echo    rusty interactive
echo.
echo  To start in learning mode:
echo    rusty learning
echo.
echo  For help:
echo    rusty --help
echo.
echo  NOTE: If 'rusty' command is not found, please:
echo    1. Restart your terminal/command prompt
echo    2. Or manually add %INSTALL_DIR% to your PATH
echo.
echo ========================================
echo.

pause
endlocal

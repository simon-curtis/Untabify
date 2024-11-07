#!/usr/bin/env pwsh
param(
  # Specify the exact version of Untabify to install.
  [String]$Version = "latest",
  # Forces installing the baseline build regardless of what CPU you are actually using.
  [Switch]$ForceBaseline = $false,
  # Skips installing powershell completions to your profile
  [Switch]$NoCompletions = $false,
  # Debugging: Always download with 'Invoke-RestMethod' instead of 'curl.exe'
  [Switch]$DownloadWithoutCurl = $false
);

# filter out 32 bit + ARM
if (-not ((Get-CimInstance Win32_ComputerSystem)).SystemType -match "x64-based") {
  Write-Output "Install Failed:"
  Write-Output "Untabify for Windows is currently only available for x86 64-bit Windows.`n"
  return 1
}

# This corresponds to .win10_rs5 in build.zig
$MinBuild = 17763;
$MinBuildName = "Windows 10 1809"

$WinVer = [System.Environment]::OSVersion.Version
if ($WinVer.Major -lt 10 -or ($WinVer.Major -eq 10 -and $WinVer.Build -lt $MinBuild)) {
  Write-Warning "Untabify requires at ${MinBuildName} or newer.`n`nThe install will still continue but it may not work.`n"
  return 1
}

$ErrorActionPreference = "Stop"

function Publish-Env {
  # Constants for SendMessageTimeout
  $WM_SETTINGCHANGE = 0x1A
  $HWND_BROADCAST = [IntPtr]::Zero -bor 0xFFFF
  $SMTO_ABORTIFHUNG = 0x0002
  $timeout = 1000  # Timeout in milliseconds

# Load the SendMessageTimeout function from user32.dll
  Add-Type @"
using System;
using System.Runtime.InteropServices;
public class Win32 {
    [DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
    public static extern IntPtr SendMessageTimeout(
        IntPtr hWnd,
        uint Msg,
        UIntPtr wParam,
        string lParam,
        uint fuFlags,
        uint uTimeout,
        out UIntPtr lpdwResult
    );
}
"@

  # Broadcast the WM_SETTINGCHANGE message
  $result = [UIntPtr]::Zero
  [Win32]::SendMessageTimeout(
      $HWND_BROADCAST,
      $WM_SETTINGCHANGE,
      [UIntPtr]::Zero,
      "Environment",
      $SMTO_ABORTIFHUNG,
      $timeout,
      [ref]$result
  ) | Out-Null

  # Confirm broadcast
  if ($result -eq [UIntPtr]::Zero) {
      Write-Output "Notification sent successfully."
  } else {
      Write-Output "Failed to send notification."
  }
}

# The installation of Untabify is it's own function so that in the unlikely case the $IsBaseline check fails, we can do a recursive call.
# There are also lots of sanity checks out of fear of anti-virus software or other weird Windows things happening.
function Install-Untabify {
  param(
    [string]$Version,
    [bool]$ForceBaseline = $False
  );

  # if a semver is given, we need to adjust it to this format: untabify-v0.0.0
  if ($Version -match "^\d+\.\d+\.\d+$") {
    $Version = "untabify-v$Version"
  }
  elseif ($Version -match "^v\d+\.\d+\.\d+$") {
    $Version = "untabify-$Version"
  }

  $Arch = "x64"
  $IsBaseline = $ForceBaseline
  if (!$IsBaseline) {
    $IsBaseline = !( `
      Add-Type -MemberDefinition '[DllImport("kernel32.dll")] public static extern bool IsProcessorFeaturePresent(int ProcessorFeature);' `
        -Name 'Kernel32' -Namespace 'Win32' -PassThru `
    )::IsProcessorFeaturePresent(40);
  }

  $Root = if ($env:UNTABIFY_INSTALL) { $env:UNTABIFY_INSTALL } else { "${Home}\bin\untabify" }
  mkdir -Force "${Root}" | Out-Null
  $ExePath = "${Root}\untabify.exe"

  try {
    Remove-Item $ExePath -Force
  } catch [System.Management.Automation.ItemNotFoundException] {
    # ignore
  } catch {
    Write-Output "Install Failed - An unknown error occurred while trying to remove the existing installation"
    Write-Output $_
    return 1
  }

  $Target = "untabify-windows-$Arch"
  if ($IsBaseline) {
    $Target = "untabify-windows-$Arch-baseline"
  }
  $BaseURL = "https://github.com/simon-curtis/untabify/releases"
  $URL = "$BaseURL/$(if ($Version -eq "latest") { "latest/download" } else { "download/$Version" })/$Target.exe"

  # curl.exe is faster than PowerShell 5's 'Invoke-WebRequest'
  # note: 'curl' is an alias to 'Invoke-WebRequest'. so the exe suffix is required
  if (-not $DownloadWithoutCurl) {
    # if we have curl installed
    curl.exe -#SfLo "$ExePath" "$URL" 
  }
  if ($DownloadWithoutCurl -or ($LASTEXITCODE -ne 0)) {
    Write-Warning "The command 'curl.exe $URL -o $ExePath' exited with code ${LASTEXITCODE}`nTrying an alternative download method..."
    try {
      # Use Invoke-RestMethod instead of Invoke-WebRequest because Invoke-WebRequest breaks on
      # some machines, see 
      Invoke-RestMethod -Uri $URL -OutFile $ExePath
    } catch {
      Write-Output "Install Failed - could not download $URL"
      Write-Output "The command 'Invoke-RestMethod $URL -OutFile $ExePath' exited with code ${LASTEXITCODE}`n"
      return 1
    }
  }

  if (!(Test-Path $ExePath)) {
    Write-Output "Install Failed - could not download $URL"
    Write-Output "The file '$ExePath' does not exist. Did an antivirus delete it?`n"
    return 1
  }

  if ($LASTEXITCODE -eq 1073741795) { # STATUS_ILLEGAL_INSTRUCTION
    if ($IsBaseline) {
      Write-Output "Install Failed - untabify.exe (baseline) is not compatible with your CPU.`n"
      Write-Output "Please open a GitHub issue with your CPU model:`nhttps://github.com/simon-curtis/Untabify/issues/new/choose`n"
      return 1
    }

    Write-Output "Install Failed - untabify.exe is not compatible with your CPU. This should have been detected before downloading.`n"
    Write-Output "Attempting to download untabify.exe (baseline) instead.`n"

    Install-Untabify -Version $Version -ForceBaseline $True
    return 1
  }
  # '-1073741515' was spotted in the wild, but not clearly documented as a status code:
  # https://discord.com/channels/876711213126520882/1149339379446325248/1205194965383250081
  # http://community.sqlbackupandftp.com/t/error-1073741515-solved/1305
  if (($LASTEXITCODE -eq 3221225781) -or ($LASTEXITCODE -eq -1073741515)) # STATUS_DLL_NOT_FOUND
  { 
    Write-Output "Install Failed - You are missing a DLL required to run untabify.exe"
    Write-Output "This can be solved by installing the Visual C++ Redistributable from Microsoft:`nSee https://learn.microsoft.com/cpp/windows/latest-supported-vc-redist`nDirect Download -> https://aka.ms/vs/17/release/vc_redist.x64.exe`n`n"
    Write-Output "The command '${Root}\untabify.exe --version' exited with code ${LASTEXITCODE}`n"
    return 1
  }
  if ($LASTEXITCODE -ne 0) {
    Write-Output "Install Failed - could not verify untabify.exe"
    Write-Output "The command '${Root}\untabify.exe --version' exited with code ${LASTEXITCODE}`n"
    return 1
  }

  try {
    $env:IS_UNTABIFY_AUTO_UPDATE = "1"
    # TODO: When powershell completions are added, make this switch actually do something
    if ($NoCompletions) {
      $env:UNTABIFY_NO_INSTALL_COMPLETIONS = "1"
    }
    # It also installs completions.
    $output = "$(& $ExePath completions 2>&1)"
    if ($LASTEXITCODE -ne 0) {
      Write-Output $output
      Write-Output "Install Failed - could not finalize installation"
      Write-Output "The command '${Root}\untabify.exe completions' exited with code ${LASTEXITCODE}`n"
      return 1
    }
  } catch {
    # it is possible on powershell 5 that an error happens, but it is probably fine?
  }
  $env:IS_UNTABIFY_AUTO_UPDATE = $null
  $env:UNTABIFY_NO_INSTALL_COMPLETIONS = $null

  $C_RESET = [char]27 + "[0m"
  $C_GREEN = [char]27 + "[1;32m"

  Write-Output "${C_GREEN}Untabify $(& $ExePath --version) was installed successfully!${C_RESET}"
  Write-Output "The binary is located at ${Root}\untabify.exe`n"

  $existing = Get-Command untabify -ErrorAction SilentlyContinue
  if ($null -ne $existing -and $existing.source -ne $ExePath) {
    Write-Warning "Note: Another untabify.exe is already in %PATH% at $($existing.source)`nTyping 'untabify' in your terminal will not use what was just installed.`n"
  }
  else {
    # Only try adding to path if there isn't already a untabify.exe in the path
    $Path = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($Path -notcontains $Root) {
      $Path = (("$Path;$Root" -split ";") | Select-Object -Unique) -join ";"
      [Environment]::SetEnvironmentVariable("PATH", $Path, "User")

      # $machinePath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
      # $env:Path = (("$machinePath;$Path;$env:Path" -split ";") | Select-Object -Unique) -join ";"
      Publish-Env
    }

    Write-Output "To get started, restart your terminal/editor, then type `"untabify`"`n"
  }

  $LASTEXITCODE = 0;
}

Install-Untabify -Version $Version -ForceBaseline $ForceBaseline
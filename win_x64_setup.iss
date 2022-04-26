#define Ver "0.2.1"
[Setup]
AppId={{1B032A50-FB74-48AF-A3E9-BDBF50CA7DBF}
AppName=vespa
AppVersion={#Ver}
AppVerName=vespa{#Ver}
DefaultDirName={commonpf}\vespa
DefaultGroupName=vespa
OutputDir=.
OutputBaseFilename=vespa{#Ver}_win_x64
Compression=lzma
SolidCompression=yes
PrivilegesRequired=none
DisableWelcomePage=no
ChangesAssociations=yes
ArchitecturesInstallIn64BitMode=x64
ArchitecturesAllowed=x64

[Languages]
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"

[Registry]
Root: HKCR; Subkey: "Directory\shell\vespa"; ValueType: string; ValueData: "Run Vespa Here"; Flags: uninsdeletekey;
Root: HKCR; Subkey: "Directory\shell\vespa\command"; ValueType: string; ValueData: "cmd /K ""{app}\vespa.exe"" -d %1 & echo press enter to exit & pause > nul & exit"; Flags: uninsdeletekey;
Root: HKCR; Subkey: "Directory\Background\shell\vespa"; ValueType: string; ValueData: "Run Vespa Here"; Flags: uninsdeletekey;
Root: HKCR; Subkey: "Directory\Background\shell\vespa\command"; ValueType: string; ValueData: "cmd /K ""{app}\vespa.exe"" -d %v & echo press enter to exit & pause > nul & exit"; Flags: uninsdeletekey;

[Files]
Source: ".\target\x86_64-pc-windows-msvc\release\vespa.exe"; DestDir: "{app}"; Flags: ignoreversion

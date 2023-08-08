using System.Runtime.Versioning;
using Microsoft.Win32;

namespace SlimeVRInstaller.Installer
{
    [SupportedOSPlatform("windows")]
    public static class RegistryUtils
    {
        public static string? WebViewVersion
        {
            get
            {
                if (Environment.Is64BitOperatingSystem)
                {
                    return GetWebViewVersion(Registry.LocalMachine.OpenSubKey("SOFTWARE\\WOW6432Node\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}")) ??
                        GetWebViewVersion(Registry.CurrentUser.OpenSubKey("Software\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"));
                }
                else
                {
                    return GetWebViewVersion(Registry.LocalMachine.OpenSubKey("SOFTWARE\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}")) ??
                        GetWebViewVersion(Registry.CurrentUser.OpenSubKey("Software\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"));
                }
            }
        }

        public static bool IsWebViewInstalled => WebViewVersion != null;

        private static string? GetWebViewVersion(RegistryKey? key)
        {
            if (key == null) return null;

            var regVal = key.GetValue("pv");
            if (regVal != null && regVal is string regValString && !string.IsNullOrWhiteSpace(regValString) && regValString != "0.0.0.0")
            {
                return regValString;
            }
            return null;
        }

        public static bool IsSlimeVRInstalled => Registry.LocalMachine.OpenSubKey("Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\SlimeVR") != null;

        public static void RegisterInstallation(string installPath)
        {
            var regKey = Registry.LocalMachine.CreateSubKey("Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\SlimeVR");
            regKey.SetValue("InstallLocation", installPath);
            regKey.SetValue("DisplayName", "SlimeVR");
            regKey.SetValue("UninstallString", $"\"{installPath}\\uninstall.exe\"");
            regKey.SetValue("DisplayIcon", $"{installPath}\\slimevr.exe");
            regKey.SetValue("HelpLink", "https://docs.slimevr.dev/");
            regKey.SetValue("URLInfoAbout", "https://slimevr.dev/");
            regKey.SetValue("URLUpdateInfo", "https://github.com/SlimeVR/SlimeVR-Installer/releases");
        }

        public static void RegisterInstallDate()
        {
            var regKey = Registry.LocalMachine.CreateSubKey("Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\SlimeVR");
            regKey.SetValue("InstallDate", DateTime.Now.ToString("yyyyMMdd"));
        }
    }
}

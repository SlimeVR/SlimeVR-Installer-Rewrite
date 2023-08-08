using System.Runtime.InteropServices;
using System.Runtime.Versioning;
using Microsoft.Win32;

namespace SlimeVRInstaller.Installer.InstallHandlers
{
    public class WebViewInstallHandler : ExeInstallHandler
    {
        [SupportedOSPlatform("windows")]
        public static string? WebViewVersion
        {
            get
            {
                if (Environment.Is64BitOperatingSystem)
                {
                    return GetWebViewVersion("HKEY_LOCAL_MACHINE\\SOFTWARE\\WOW6432Node\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}") ??
                        GetWebViewVersion("HKEY_CURRENT_USER\\Software\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}");
                }
                else
                {
                    return GetWebViewVersion("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}") ??
                        GetWebViewVersion("HKEY_CURRENT_USER\\Software\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}");
                }
            }
        }

        [SupportedOSPlatform("windows")]
        public static bool IsWebViewInstalled => WebViewVersion != null;

        public WebViewInstallHandler(string name, string version, string uri, string fileName) : base(name, version, uri, fileName)
        {
        }

        [SupportedOSPlatform("windows")]
        private static string? GetWebViewVersion(string path)
        {
            var regVal = Registry.GetValue(path, "pv", null);
            if (regVal != null && regVal is string regValString && !string.IsNullOrWhiteSpace(regValString) && regValString != "0.0.0.0")
            {
                return regValString;
            }
            return null;
        }

        public override bool NeedsInstall()
        {
            if (!RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                Console.WriteLine("OS is not Windows, WebView2 is not required.");
                return false;
            }
            var webViewVersion = WebViewVersion;
            if (webViewVersion != null)
            {
                Console.WriteLine($"WebView2 ({webViewVersion}) is already installed.");
                return false;
            }
            return true;
        }

        public override async Task Install(CancellationToken cancellationToken)
        {
            await base.Install(cancellationToken);
        }
    }
}

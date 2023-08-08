using System.Runtime.InteropServices;

namespace SlimeVRInstaller.Installer.InstallHandlers
{
    public class WebViewInstallHandler : ExeInstallHandler
    {
        public WebViewInstallHandler(string name, string version, string uri, string fileName) : base(name, version, uri, fileName)
        {
        }

        public override bool NeedsInstall()
        {
            if (!RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                Console.WriteLine("OS is not Windows, WebView2 is not required.");
                return false;
            }
            var webViewVersion = RegistryUtils.WebViewVersion;
            if (webViewVersion != null)
            {
                Console.WriteLine($"WebView2 ({webViewVersion}) is already installed.");
                return false;
            }
            return true;
        }

        public override async Task Install(string installPath, CancellationToken cancellationToken)
        {
            await base.Install(installPath, cancellationToken);
        }
    }
}

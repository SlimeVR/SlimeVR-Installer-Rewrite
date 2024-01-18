using System.Runtime.InteropServices;
using SlimeVRInstaller.Installer.Components;

namespace SlimeVRInstaller.Installer
{
    public class ComponentState
    {
        public readonly InstallComponent Component;
        public DirectoryInfo TempFolder;
        public DirectoryInfo InstallFolder;

        public bool IsInstalled;
        public FileInfo? File = null;

        public bool DoesFileExist => File != null && File.Exists;

        public ComponentState(
            InstallComponent component,
            DirectoryInfo tempPath,
            DirectoryInfo installPath
        )
        {
            Component = component;
            TempFolder = tempPath;
            InstallFolder = installPath;
            IsInstalled = IsComponentInstalled(component.Type);
        }

        public static bool IsComponentInstalled(InstallComponentType component)
        {
            switch (component)
            {
                case InstallComponentType.WebView:
                    if (!RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
                    {
                        Console.WriteLine("OS is not Windows, WebView2 is not required.");
                        return true;
                    }
                    var webViewVersion = RegistryUtils.WebViewVersion;
                    if (webViewVersion != null)
                    {
                        Console.WriteLine($"WebView2 ({webViewVersion}) is already installed.");
                        return true;
                    }
                    return false;
                default:
                    return false;
            }
        }
    }
}

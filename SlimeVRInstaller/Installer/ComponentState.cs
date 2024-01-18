using SlimeVRInstaller.Installer.Components;

namespace SlimeVRInstaller.Installer
{
    public class ComponentState
    {
        public readonly InstallComponent Component;
        public DirectoryInfo TempFolder;
        public DirectoryInfo InstallFolder;

        public FileInfo? File = null;
        public bool IsInstalled;

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
            IsInstalled = component.IsInstalled;
        }
    }
}

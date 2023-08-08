namespace SlimeVRInstaller.Installer.InstallHandlers
{
    public class ZipInstallHandler : InstallHandler
    {
        public ZipInstallHandler(string name, string version, string uri, string fileName) : base(name, version, uri, fileName)
        {
        }

        public override async Task Install(string installPath, CancellationToken cancellationToken = default)
        {
            await base.Install(installPath, cancellationToken);
            //ZipFile.ExtractToDirectory(FileName, installPath, true);
        }
    }
}

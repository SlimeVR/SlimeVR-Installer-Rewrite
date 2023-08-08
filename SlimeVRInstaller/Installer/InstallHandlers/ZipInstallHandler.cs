using System.IO.Compression;

namespace SlimeVRInstaller.Installer.InstallHandlers
{
    public class ZipInstallHandler : InstallHandler
    {
        public readonly Action<string, string, CancellationToken> AfterUnzipAction;

        public ZipInstallHandler(string name, string version, string uri, string fileName, Action<string, string, CancellationToken> afterUnzipAction) : base(name, version, uri, fileName)
        {
            AfterUnzipAction = afterUnzipAction;
        }

        public override async Task Install(string tempPath, string installPath, CancellationToken cancellationToken = default)
        {
            await base.Install(tempPath, installPath, cancellationToken);

            var zipTempPath = Path.Combine(tempPath, Path.GetRandomFileName());
            var zipTemp = Directory.CreateDirectory(zipTempPath);
            ZipFile.ExtractToDirectory(FileName, zipTemp.FullName);

            AfterUnzipAction.Invoke(tempPath, installPath, cancellationToken);
        }
    }
}

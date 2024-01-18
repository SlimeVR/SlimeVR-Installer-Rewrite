using System.IO.Compression;

namespace SlimeVRInstaller.Installer.Handlers
{
    public class ZipInstallHandler : InstallHandler
    {
        public readonly Action<DirectoryInfo, DirectoryInfo, CancellationToken> AfterUnzipAction;

        public ZipInstallHandler(
            Action<DirectoryInfo, DirectoryInfo, CancellationToken> afterUnzipAction
        )
        {
            AfterUnzipAction = afterUnzipAction;
        }

        public override async Task Install(
            ComponentState state,
            CancellationToken cancellationToken = default
        )
        {
            await base.Install(state, cancellationToken);

            var zipTempPath = Path.Combine(state.TempFolder.FullName, Path.GetRandomFileName());
            var zipTemp = Directory.CreateDirectory(zipTempPath);
            ZipFile.ExtractToDirectory(state.File!.FullName, zipTemp.FullName);

            AfterUnzipAction.Invoke(zipTemp, state.InstallFolder, cancellationToken);
        }
    }
}

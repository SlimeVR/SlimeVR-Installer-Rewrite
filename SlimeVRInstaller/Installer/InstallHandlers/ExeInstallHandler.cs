using System.Diagnostics;
using SlimeVRInstaller.Installer.Exceptions;

namespace SlimeVRInstaller.Installer.InstallHandlers
{
    public class ExeInstallHandler : InstallHandler
    {
        public ExeInstallHandler(string name, string version, string uri, string fileName) : base(name, version, uri, fileName)
        {
        }

        public override async Task Install(CancellationToken cancellationToken)
        {
            var process = new Process();
            process.StartInfo.FileName = DownloadedFilePath;
            process.Start();
            await process.WaitForExitAsync(cancellationToken);

            if (process.ExitCode != 0)
            {
                throw new FailedToInstallException($"Failed to install {Name}: \"{DownloadedFilePath}\" exited with code {process.ExitCode}.");
            }
        }
    }
}

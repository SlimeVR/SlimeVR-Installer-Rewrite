using System.Diagnostics;
using SlimeVRInstaller.Installer.Exceptions;

namespace SlimeVRInstaller.Installer.InstallHandlers
{
    public class ExeInstallHandler : InstallHandler
    {
        public ExeInstallHandler(string name, string version, string uri, string fileName) : base(name, version, uri, fileName)
        {
        }

        public override async Task Install(string filePath, CancellationToken cancellationToken)
        {
            var process = new Process();
            process.StartInfo.FileName = filePath;
            process.Start();
            await process.WaitForExitAsync(cancellationToken);

            if (process.ExitCode != 0)
            {
                throw new FailedToInstallException($"Failed to install {Name}: \"{filePath}\" exited with code {process.ExitCode}.");
            }
        }
    }
}

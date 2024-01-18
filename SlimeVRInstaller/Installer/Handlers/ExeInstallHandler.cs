using System.Diagnostics;
using SlimeVRInstaller.Installer.Exceptions;

namespace SlimeVRInstaller.Installer.Handlers
{
    public class ExeInstallHandler : InstallHandler
    {
        public override async Task Install(
            ComponentState state,
            CancellationToken cancellationToken
        )
        {
            await base.Install(state, cancellationToken);

            var process = new Process();
            process.StartInfo.FileName = state.File!.FullName;
            process.Start();
            await process.WaitForExitAsync(cancellationToken);

            if (process.ExitCode != 0)
            {
                throw new InstallException(
                    $"Failed to install {state.Component.Name}: \"{state.File}\" exited with code {process.ExitCode}."
                );
            }
        }
    }
}

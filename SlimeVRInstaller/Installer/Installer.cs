using SlimeVRInstaller.Installer.Components;
using SlimeVRInstaller.Network;

namespace SlimeVRInstaller.Installer
{
    public class Installer : IDisposable
    {
        public readonly HttpClient httpClient = new();

        public static readonly string DefaultInstallPath = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.ProgramFilesX86),
            "SlimeVR Server"
        );

        public void InstallAllNeeded()
        {
            InstallComponents(Enum.GetValues<InstallComponentType>());
        }

        public void InstallComponents(InstallComponentType[] components)
        {
            // Create a temporary directory to download files to
            var tempFolder = Directory.CreateTempSubdirectory("SlimeVR-Installer-");
            Console.WriteLine($"Using temp directory \"{tempFolder.FullName}\" for installation.");

            var installPath = Directory.CreateDirectory(DefaultInstallPath);

            ComponentState[] states = components
                .Select(c => new ComponentState(
                    InstallComponent.Components[c],
                    tempFolder,
                    installPath
                ))
                .ToArray();

            // Download files asynchronously
            var tasks = new Task[states.Length];
            for (int i = 0; i < states.Length; i++)
            {
                tasks[i] = Download(states[i], tempFolder);
            }

            // Wait for downloads to finish
            Task.WaitAll(tasks);

            // Install each component one at a time
            foreach (var state in states)
            {
                state.Component.InstallHandler.Install(state).Wait();
            }

            // Remove the temporary directory when done
            Console.WriteLine("Done installing. Press any key to delete the temp directory...");
            Console.ReadLine();
            tempFolder.Delete(recursive: true);
        }

        public async Task Download(
            ComponentState state,
            DirectoryInfo tempFolder,
            CancellationToken cancellationToken = default
        )
        {
            // Check if the file is already downloaded
            if (state.DoesFileExist)
                return;

            var progressHandler = new Progress<DownloadProgress>(progress =>
                Console.WriteLine(
                    $"{state.Component.Name} [{progress.BytesDownloadedMiB:0.00} MiB / {progress.TotalBytesMiB:0.00} MiB] ({progress.Progress:0.00%})"
                )
            );

            var filePath = Path.Combine(tempFolder.FullName, state.Component.FileName);
            state.File = new FileInfo(filePath);
            await DownloadUtils.Download(
                httpClient,
                state.Component.LatestUri,
                filePath,
                progressHandler,
                cancellationToken
            );
        }

        public void Dispose()
        {
            httpClient.Dispose();
            GC.SuppressFinalize(this);
        }
    }
}

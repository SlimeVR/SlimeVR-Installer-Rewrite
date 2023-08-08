using SlimeVRInstaller.Installer.Exceptions;
using SlimeVRInstaller.Network;

namespace SlimeVRInstaller.Installer.InstallHandlers
{
    public class InstallHandler
    {
        public readonly string Name;
        public readonly string Version;
        public readonly string Uri;
        public readonly string FileName;

        public readonly Progress<DownloadProgress> ProgressReporter;

        public bool ShouldInstall = true;
        public string DownloadedFilePath = "";
        public bool FileExists => !string.IsNullOrWhiteSpace(DownloadedFilePath) && File.Exists(DownloadedFilePath);

        public InstallHandler(string name, string version, string uri, string fileName)
        {
            Name = name;
            Version = version;
            Uri = uri;
            FileName = fileName;

            ProgressReporter = new(progress => Console.WriteLine($"{Name} [{progress.BytesDownloadedMiB:0.00} MiB / {progress.TotalBytesMiB:0.00} MiB] ({progress.Progress:0.00%})"));
        }

        public void CheckForFile()
        {
            if (!FileExists)
            {
                throw new InstallException($"{nameof(DownloadedFilePath)} is not set or does not exist. Download the file and set the downloaded file path before installing.");
            }
        }

        public virtual bool NeedsInstall()
        {
            return true;
        }

        public virtual async Task Install(CancellationToken cancellationToken = default)
        {
            if (!ShouldInstall) return;
            CheckForFile();
        }
    }
}

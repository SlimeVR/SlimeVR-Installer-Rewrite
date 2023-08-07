using SlimeVRInstaller.Network;

namespace SlimeVRInstaller.Installer
{
    public class InstallFile
    {
        public readonly string Name;
        public readonly string Version;
        public readonly string Uri;
        public readonly string FileName;

        public readonly Progress<DownloadProgress> ProgressReporter;

        public InstallFile(string name, string version, string uri, string fileName)
        {
            Name = name;
            Version = version;
            Uri = uri;
            FileName = fileName;

            ProgressReporter = new(progress => Console.WriteLine($"{Name} [{progress.BytesDownloadedMiB:0.00} MiB / {progress.TotalBytesMiB:0.00} MiB] ({progress.Progress:0.00%})"));
        }
    }
}

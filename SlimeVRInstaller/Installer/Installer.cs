using SlimeVRInstaller.Network;

namespace SlimeVRInstaller.Installer
{
    public class Installer : IDisposable
    {
        public readonly HttpClient httpClient = new();

        // Server
        public static readonly InstallFile Server = new("SlimeVR Server", "", "https://github.com/SlimeVR/SlimeVR-Server/releases/latest/download/SlimeVR-win64.zip", "SlimeVR-win64.zip");
        public static readonly InstallFile Java = new("Java JRE", "", "https://github.com/adoptium/temurin17-binaries/releases/download/jdk-17.0.4.1%2B1/OpenJDK17U-jre_x64_windows_hotspot_17.0.4.1_1.zip", "OpenJDK17U-jre_x64_windows_hotspot_17.0.4.1_1.zip");
        public static readonly InstallFile WebView = new("Edge WebView2", "", "https://go.microsoft.com/fwlink/p/?LinkId=2124703", "MicrosoftEdgeWebView2RuntimeInstaller.exe");

        // Driver
        public static readonly InstallFile SteamVR = new("SteamVR Driver", "", "https://github.com/SlimeVR/SlimeVR-OpenVR-Driver/releases/latest/download/slimevr-openvr-driver-win64.zip", "slimevr-openvr-driver-win64.zip");

        // Feeder
        public static readonly InstallFile Feeder = new("SlimeVR Feeder App", "", "https://github.com/SlimeVR/SlimeVR-Feeder-App/releases/latest/download/SlimeVR-Feeder-App-win64.zip", "SlimeVR-Feeder-App-win64.zip");

        public void Install()
        {
            // Create a temporary directory to download files to
            var tempFolder = Directory.CreateTempSubdirectory("SlimeVR-Installer-");
            Console.WriteLine($"Using temp directory \"{tempFolder.FullName}\" for installation");

            // Download files asynchronously
            var tasks = new Task[]
            {
                Download(Server, tempFolder.FullName),
                Download(Java, tempFolder.FullName),
                Download(WebView, tempFolder.FullName),
                Download(SteamVR, tempFolder.FullName),
                Download(Feeder, tempFolder.FullName),
            };

            // Wait for downloads to finish
            Task.WaitAll(tasks);

            // Remove the temporary directory when done
            Console.WriteLine("Done downloading. Press any key to delete the temp directory...");
            Console.ReadLine();
            tempFolder.Delete(recursive: true);
        }

        public async Task Download(InstallFile installFile, string tempFolderPath, CancellationToken cancellationToken = default)
        {
            await DownloadUtils.Download(httpClient, installFile.Uri, Path.Combine(tempFolderPath, installFile.FileName), installFile.ProgressReporter, cancellationToken);
        }

        public void Dispose()
        {
            httpClient.Dispose();
            GC.SuppressFinalize(this);
        }
    }
}

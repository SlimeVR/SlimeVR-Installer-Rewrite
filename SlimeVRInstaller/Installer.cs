using SlimeVRInstaller.Network;

namespace SlimeVRInstaller
{
    public class Installer : IDisposable
    {
        public readonly HttpClient httpClient = new();

        // Server
        public static readonly string ServerUrl = "https://github.com/SlimeVR/SlimeVR-Server/releases/latest/download/SlimeVR-win64.zip";
        public static readonly string JavaUrl = "https://github.com/adoptium/temurin17-binaries/releases/download/jdk-17.0.4.1%2B1/OpenJDK17U-jre_x64_windows_hotspot_17.0.4.1_1.zip";
        public static readonly string WebViewUrl = "https://go.microsoft.com/fwlink/p/?LinkId=2124703";

        // Driver
        public static readonly string SteamVRDriverUrl = "https://github.com/SlimeVR/SlimeVR-OpenVR-Driver/releases/latest/download/slimevr-openvr-driver-win64.zip";

        // Feeder
        public static readonly string FeederAppUrl = "https://github.com/SlimeVR/SlimeVR-Feeder-App/releases/latest/download/SlimeVR-Feeder-App-win64.zip";

        public void Install()
        {
            // Create a temporary directory to download files to
            var tempFolder = Directory.CreateTempSubdirectory("SlimeVR-Installer-");
            Console.WriteLine($"Using temp directory \"{tempFolder.FullName}\" for installation");

            // Download files asynchronously
            var downloadProgress = new Progress<DownloadProgress>(progress => Console.WriteLine($"Download is at {progress.Progress:0.00%}"));
            var tasks = new Task[]
            {
                DownloadUtils.Download(httpClient, ServerUrl, Path.Combine(tempFolder.FullName, "SlimeVR-win64.zip"), downloadProgress),
                DownloadUtils.Download(httpClient, JavaUrl, Path.Combine(tempFolder.FullName, "OpenJDK17U-jre_x64_windows_hotspot_17.0.4.1_1.zip"), downloadProgress),
                DownloadUtils.Download(httpClient, WebViewUrl, Path.Combine(tempFolder.FullName, "MicrosoftEdgeWebView2RuntimeInstaller.exe"), downloadProgress),
                DownloadUtils.Download(httpClient, SteamVRDriverUrl, Path.Combine(tempFolder.FullName, "slimevr-openvr-driver-win64.zip"), downloadProgress),
                DownloadUtils.Download(httpClient, FeederAppUrl, Path.Combine(tempFolder.FullName, "SlimeVR-Feeder-App-win64.zip"), downloadProgress),
            };

            // Wait for downloads to finish
            Task.WaitAll(tasks);

            // Remove the temporary directory when done
            Console.WriteLine("Done downloading. Press any key to delete.");
            Console.ReadLine();
            tempFolder.Delete(recursive: true);
        }

        public void Dispose()
        {
            httpClient.Dispose();
            GC.SuppressFinalize(this);
        }
    }
}

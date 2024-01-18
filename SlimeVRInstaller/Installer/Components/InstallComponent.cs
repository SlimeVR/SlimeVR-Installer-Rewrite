using SlimeVRInstaller.Installer.Handlers;

namespace SlimeVRInstaller.Installer.Components
{
    public class InstallComponent
    {
        public readonly InstallComponentType Type;

        public readonly string Name;
        public readonly string LatestUri;
        public readonly string FileName;

        public readonly InstallComponentType[] Requirements;

        public readonly InstallHandler InstallHandler;

        public static readonly Dictionary<InstallComponentType, InstallComponent> Components =
            new();

        public static readonly InstallComponent Server =
            new(
                InstallComponentType.Server,
                "SlimeVR Server",
                "https://github.com/SlimeVR/SlimeVR-Server/releases/latest/download/SlimeVR-win64.zip",
                "SlimeVR-win64.zip",
                new InstallComponentType[]
                {
                    InstallComponentType.Java,
                    InstallComponentType.WebView
                },
                new ZipInstallHandler((zipFolder, installFolder, cancellationToken) => { })
            );
        public static readonly InstallComponent Java =
            new(
                InstallComponentType.Java,
                "Java JRE",
                "https://github.com/adoptium/temurin17-binaries/releases/download/jdk-17.0.4.1%2B1/OpenJDK17U-jre_x64_windows_hotspot_17.0.4.1_1.zip",
                "OpenJDK17U-jre_x64_windows_hotspot_17.0.4.1_1.zip",
                Array.Empty<InstallComponentType>(),
                new ZipInstallHandler((zipFolder, installFolder, cancellationToken) => { })
            );
        public static readonly InstallComponent WebView =
            new(
                InstallComponentType.WebView,
                "Edge WebView2",
                "https://go.microsoft.com/fwlink/p/?LinkId=2124703",
                "MicrosoftEdgeWebView2RuntimeInstaller.exe",
                Array.Empty<InstallComponentType>(),
                new ExeInstallHandler()
            );
        public static readonly InstallComponent SteamVR =
            new(
                InstallComponentType.SteamVR,
                "SteamVR Driver",
                "https://github.com/SlimeVR/SlimeVR-OpenVR-Driver/releases/latest/download/slimevr-openvr-driver-win64.zip",
                "slimevr-openvr-driver-win64.zip",
                Array.Empty<InstallComponentType>(),
                new ZipInstallHandler((zipFolder, installFolder, cancellationToken) => { })
            );
        public static readonly InstallComponent Feeder =
            new(
                InstallComponentType.Feeder,
                "SlimeVR Feeder App",
                "https://github.com/SlimeVR/SlimeVR-Feeder-App/releases/latest/download/SlimeVR-Feeder-App-win64.zip",
                "SlimeVR-Feeder-App-win64.zip",
                Array.Empty<InstallComponentType>(),
                new ZipInstallHandler((zipFolder, installFolder, cancellationToken) => { })
            );

        public InstallComponent(
            InstallComponentType component,
            string name,
            string uri,
            string fileName,
            InstallComponentType[] requirements,
            InstallHandler installHandler
        )
        {
            Type = component;
            Name = name;
            LatestUri = uri;
            FileName = fileName;
            Requirements = requirements;
            InstallHandler = installHandler;

            Components.Add(component, this);
        }
    }
}

namespace SlimeVRInstaller.Network
{
    public readonly struct DownloadProgress
    {
        public readonly long BytesDownloaded;
        public readonly long TotalBytes;
        public readonly float Progress;

        public DownloadProgress(long bytesDownloaded, long totalBytes)
        {
            BytesDownloaded = bytesDownloaded;
            TotalBytes = totalBytes;
            // Handle 0 divider values and clamp to the 0 - 1 range
            Progress = totalBytes > 0L ? MathF.Min((float)bytesDownloaded / totalBytes, 1f) : 0f;
        }

        public DownloadProgress(float progress)
        {
            BytesDownloaded = -1L;
            TotalBytes = -1L;
            Progress = progress;
        }
    }
}

using System.Buffers;

namespace SlimeVRInstaller.Network
{
    public static class DownloadUtils
    {
        public static Uri CreateUri(string uri)
        {
            return new Uri(uri, UriKind.RelativeOrAbsolute);
        }

        public static async Task Download(HttpClient httpClient, string uri, string targetFilePath, IProgress<float>? progress = null, CancellationToken cancellationToken = default)
        {
            await Download(httpClient, CreateUri(uri), targetFilePath, progress, cancellationToken);
        }

        public static async Task Download(HttpClient httpClient, Uri uri, string targetFilePath, IProgress<float>? progress = null, CancellationToken cancellationToken = default)
        {
            // Create the target file from the download
            using var targetFile = new FileStream(targetFilePath, FileMode.Create);

            // Only request the headers for now, content will be downloaded separately
            using var response = await httpClient.GetAsync(uri, HttpCompletionOption.ResponseHeadersRead, cancellationToken);

            // Set up the content to be read
            var contentLength = response.Content.Headers.ContentLength;
            using var content = await response.Content.ReadAsStreamAsync(cancellationToken);

            // When no progress reporter was given or the content length is unknown
            if (progress == null || contentLength == null)
            {
                // Download without progress reporting
                await content.CopyToAsync(targetFile, cancellationToken);
            }
            else
            {
                // Convert absolute progress (bytes downloaded) into relative progress (0 - 1)
                var relativeProgress = new Progress<long>(totalBytes => progress.Report((float)totalBytes / contentLength.Value));
                // Use the extension method to report progress while downloading
                await content.CopyToAsync(targetFile, progress: relativeProgress, cancellationToken: cancellationToken);
            }

            // Report 100% progress on completing the download
            progress?.Report(1f);
        }

        public static async Task CopyToAsync(this Stream source, Stream destination, int bufferSize = 81920, IProgress<long>? progress = null, CancellationToken cancellationToken = default)
        {
            if (bufferSize < 0)
                throw new ArgumentOutOfRangeException(nameof(bufferSize));
            if (source is null)
                throw new ArgumentNullException(nameof(source));
            if (!source.CanRead)
                throw new InvalidOperationException($"\"{nameof(source)}\" is not readable.");
            if (destination == null)
                throw new ArgumentNullException(nameof(destination));
            if (!destination.CanWrite)
                throw new InvalidOperationException($"\"{nameof(destination)}\" is not writable.");

            var buffer = ArrayPool<byte>.Shared.Rent(bufferSize);
            try
            {
                long totalBytesRead = 0;
                int currentBytesRead;
                while ((currentBytesRead = await source.ReadAsync(buffer, cancellationToken).ConfigureAwait(false)) > 0)
                {
                    await destination.WriteAsync(buffer.AsMemory(0, currentBytesRead), cancellationToken).ConfigureAwait(false);
                    totalBytesRead += currentBytesRead;
                    progress?.Report(totalBytesRead);
                }
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(buffer);
            }
        }
    }
}

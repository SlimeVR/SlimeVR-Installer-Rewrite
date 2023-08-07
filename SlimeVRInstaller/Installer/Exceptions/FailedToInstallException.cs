using System.Runtime.Serialization;

namespace SlimeVRInstaller.Installer.Exceptions
{
    public class FailedToInstallException : Exception
    {
        public FailedToInstallException()
        {
        }

        public FailedToInstallException(string? message) : base(message)
        {
        }

        public FailedToInstallException(string? message, Exception? innerException) : base(message, innerException)
        {
        }

        protected FailedToInstallException(SerializationInfo info, StreamingContext context) : base(info, context)
        {
        }
    }
}

using System.Runtime.Serialization;

namespace SlimeVRInstaller.Installer.Exceptions
{
    public class InstallException : Exception
    {
        public InstallException()
        {
        }

        public InstallException(string? message) : base(message)
        {
        }

        public InstallException(string? message, Exception? innerException) : base(message, innerException)
        {
        }

        protected InstallException(SerializationInfo info, StreamingContext context) : base(info, context)
        {
        }
    }
}

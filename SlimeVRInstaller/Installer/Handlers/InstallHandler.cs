using SlimeVRInstaller.Installer.Exceptions;

namespace SlimeVRInstaller.Installer.Handlers
{
    public abstract class InstallHandler
    {
        protected static void CheckForFile(ComponentState state)
        {
            if (!state.DoesFileExist)
            {
                throw new InstallException(
                    $"{nameof(ComponentState.File)} is not set or does not exist. Set the file path before installing."
                );
            }
        }

        public virtual async Task Install(
            ComponentState state,
            CancellationToken cancellationToken = default
        )
        {
            CheckForFile(state);
        }
    }
}

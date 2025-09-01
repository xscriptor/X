# Xtropical Neon for gnome terminal

[xtropicalneon.sh](./xtropicalneon.sh)

This script creates a custom terminal profile named 'xtropicalneon' in Gnome Terminal with specific configurations, such as color palette, transparent background, its percentage, and sets it as the default profile.

Here is a summary of what this script does:

1. Retrieves the UUID of the current default profile using gsettings get org.gnome.Terminal.ProfilesList default.
2. Generates a new UUID for the 'xtropicalneon' profile.
3. Adds the new profile to the existing profile list using gsettings set org.gnome.Terminal.ProfilesList list.
4. Defines the characteristics of the 'xtropicalneon' profile through a series of gsettings calls, including its name, background color, transparency, and color palette.
4. Sets the newly created profile as default using gsettings set org.gnome.Terminal.ProfilesList default.
5. Finally, it prints a message indicating that the *xtropicalneon* profile has been created and is now the default profile.

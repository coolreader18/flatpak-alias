# Flatpak Alias

A simple way to make shell aliases for flatpak apps.

## Installation

Make sure cargo is installed and set up.

```sh
cargo install --git https://github.com/coolreader18/flatpak-alias
```

## Usage

Place the below somewhere in your shell startup script.

```sh
eval "$(flatpak-alias sh)"
```

Then put something like this in `~/.flatpak-aliases`:

```toml
# The alias's name; what you run in the shell
[gimp]
# The app you want to run
app = "org.gimp.GIMP"
# other arguments
branch = "stable"
arch = "x86_64"
command = "gimp-2.10"
file-forwarding = true
```

This adds an alias for the following:

```sh
flatpak run --arch=x86_64 --branch=stable --command=gimp-2.10 --file-forwarding org.gimp.GIMP
```

## License

This project is licensed under the MIT license. Please see the
[LICENSE](LICENSE) file for more details

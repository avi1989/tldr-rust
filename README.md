# tldr
A rust client for [tldr-pages](https://github.com/tldr-pages/tldr).

# Installation
This package is not available in any package manager at this time. The only way to install it is to download the correct file from the release and add it to the system path.

# Usage
`tldr [OPTIONS] [NAME]`

Arguments:

**[NAME]**:  The name of the tool you want to see the tldr page for

**[Options]**:

  -p, --platform <platform>  Specify the platform of the command.

  -u, --update               Update the TLDR cache.

  -r, --reset                Deletes the tldr cache and refreshes it.

  -v, --version              Print version.

      --cache-dir            Gets the cache directory

  -h, --help                 Print help
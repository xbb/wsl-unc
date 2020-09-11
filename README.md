# wsl-unc

Wraps wsl.exe and converts any WSL UNC path found (ex. `\\wsl$\Ubuntu\home\user`) to the corresponding local path

## Example

Calling `wsl-unc -- echo \\wsl$\Ubuntu\home\user` will produce the output:

```
/home/user
```

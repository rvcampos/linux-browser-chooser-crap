# simple-browser-chooser

## Purpose
Multi-browser, multi-profiles? Just to help my daily dirty setup (not intended to be good, just useful)

## Configuration

Given the follow configuration: 
```toml
 default = "personal"

[[profile]]
name = "personal"
exec = "/usr/bin/thorium-browser"

[[profile]]
name = "work"
exec = "/usr/bin/thorium-browser"
args = ["--user-data-dir=my-data-dir"]
domains = ["*.mydomain.com", "*.mydomain.dev", "localhost:3000"]
```

* You'll have 2 profiles:
  * personal
    * cmdline: `/usr/bin/thorium-browser %u`
  * work
    * cmdline `/usr/bin/thorium-browser --user-data-dir=my-data-dir %u`
    * every domains that:
      * ends with: `mydomain.com` or `mydomain.dev`
      * equals to: `localhost:3000`
* The default one is: `personal`
* By default, this executable will try to load the configuration file from `$HOME/.config/simple-browser-chooser/configuration.toml`
  * can be overriden by setting: `SIMPLE_BROWSER_CONFIG` variable

## Usage
```shell
# Opens mydomain.com
simple-browser-chooser https://mydomain.com

# Opens duckduckgo.com
simple-browser-chooser

# Opens mydomain.com and ignores everything else
simple-browser-chooser https://mydomain.com https://mydomain.dev
```

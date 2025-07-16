# Belladonna Sherbet

## Cloneing Public Code
Clone via https: `git clone https://github.com/VoidAndCaffeine/belladonna-sherbet.git`
Clone via ssh: `git clone git@github.com:VoidAndCaffeine/belladonna-sherbet.git`

## Cloneing All Code and Assets
This is for my own refrence as the submodule points to my private assets repository and code mirror which I will not make public.
Clone via https: `git clone --recurse-submodules https://coffee-constellations@dev.azure.com/coffee-constellations/belladonna-sherbet/_git/belladonna-sherbet`

#### Push to Public Repo
in belladonna-sherbet/ run: `git remote set-url origin --push --add git@github.com:VoidAndCaffeine/belladonna-sherbet.git`

#### Fixing Assets Submodule Detached Head
in assets/ run: `git switch master`

## Building

You can build your game

```
cargo run
```

If you want the extra dev features then you can toggle them:

```
cargo run --features dev
```

## Features

- Cargo configured according to Bevy guide with build optimizations
- [Avian](https://github.com/Jondolf/avian) physics
- Generic set of starting plugins with your games logic inside `GamePlugin`

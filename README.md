# Belladonna Sherbet

## Cloning Public Code
Clone via https: `git clone https://github.com/VoidAndCaffeine/belladonna-sherbet.git`

Clone via SSH: `git clone git@github.com:VoidAndCaffeine/belladonna-sherbet.git`

## Cloning All Code and Assets
This is for my reference, as the submodule points to my private assets repository and the link to my code mirror, neither of which I will make public.

Clone via https: `git clone --recurse-submodules https://coffee-constellations@dev.azure.com/coffee-constellations/belladonna-sherbet/_git/belladonna-sherbet`

#### Push to Public Repo
In belladonna-sherbet/ run: `git remote set-url origin --push --add git@github.com:VoidAndCaffeine/belladonna-sherbet.git`

#### Fixing Assets Submodule Detached Head
In assets/ run: `git switch master`

## Building

You can build your game

```
cargo run
```

If you want the extra dev features, then you can toggle them:

```
cargo run --features dev
```

## Features

- Cargo configured according to Bevy guide with build optimizations
- [Avian](https://github.com/Jondolf/avian) physics
- Generic set of starting plugins with your games logic inside `GamePlugin`

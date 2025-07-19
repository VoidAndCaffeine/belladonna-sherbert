# Belladonna Sherbet

## Cloning Public Code
Clone via https: 
```
git clone https://github.com/VoidAndCaffeine/belladonna-sherbet.git
```

Clone via SSH: 
```
git clone git@github.com:VoidAndCaffeine/belladonna-sherbet.git
```

## Cloning All Code and Assets
This is for my reference, as the submodule points to my private assets repository and the link to my code mirror, neither of which I will make public.

Clone via https:
```
git clone --recurse-submodules https://coffee-constellations@dev.azure.com/coffee-constellations/belladonna-sherbet/_git/belladonna-sherbet
```

#### Fixing Assets Submodule Detached Head
In assets/ run: 
```
git switch master
```

#### Push to Public Repo
Add the following to ./.git/config, replacing the [remote "origin"] already there
(I don't know how much of this config is actually necessary, specifically the duplicated `remote =` and `url =`)
```
[remote "azure"]
        url = https://coffee-constellations@dev.azure.com/coffee-constellations/belladonna-sherbet/_git/belladonna-sherbet
        fetch = +refs/heads/*:refs/remotes/origin/*
[remote "github"]
        url = git@github.com:VoidAndCaffeine/belladonna-sherbet.git
        fetch = +refs/heads/*:refs/remotes/origin/*
[remote "origin"]
        url = git@github.com:VoidAndCaffeine/belladonna-sherbet.git
        url = https://coffee-constellations@dev.azure.com/coffee-constellations/belladonna-sherbet/_git/belladonna-sherbet
        pushurl = git@ssh.dev.azure.com:v3/coffee-constellations/belladonna-sherbet/belladonna-sherbet-assets
        pushurl = git@github.com:VoidAndCaffeine/belladonna-sherbet.git
        fetch = +refs/heads/*:refs/remotes/origin/*
[branch "master"]
        remote = origin
        remote = github
        remote = azure
        merge = refs/heads/master
```

## Building

You can build your game

```
cargo run
```

If you want the extra dev features, then you can toggle them:

```
cargo run --features dev
```

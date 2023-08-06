# commands

## serve

```sh
trunk serve
```

```powershell
trunk serve
```

## gen

```sh
cargo run -p gen
```

```powershell
cargo run -p gen
```

## act

### build

```sh
mkdir -p /tmp/act-artifacts
act -j build -W .github/workflows/deploy.yml -s GITHUB_TOKEN=$(gh auth token) --artifact-server-path /tmp/act-artifacts
```

```powershell
mkdir -Force -p $Env:TMP\act-artifacts
act -j build -W .github/workflows/deploy.yml -s GITHUB_TOKEN=$(gh auth token) --artifact-server-path $Env:TMP\act-artifacts
```

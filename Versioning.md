# Versioning

Zilliqa is following [Semantic Versioning 2.0](https://semver.org).

The public Zilliqa version string is in either of the following pattern:

- `<major>.<minor>.<patch>` for release versions
- `<major>.<minor>.<patch>-<pre-release>` for pre-release versions

Table of Content

- [Versioning](#versioning)
  - [Alpha Pre-release](#alpha-pre-release)
  - [Beta Pre-release](#beta-pre-release)
  - [Release](#release)
- [Branching](#branching)
  - [Master Branch](#master-branch)
  - [Release Branch](#release-branch)
- [Tagging](#tagging)
  - [Git Tagging](#git-tagging)
  - [Docker Image Tagging](#docker-image-tagging)

## Versioning

| Release Stage     | Version Example (Incremental Order)    |
|-------------------|----------------------------------------|
| Alpha pre-release | `4.3.0-alpha.0`, `4.3.0-alpha.1`, `..` |
| Beta pre-release  | `4.3.0-beta.0`, `4.3.0-beta.1`, `...`  |
| Release           | `4.3.0`, `4.3.1`, `...`                |

### Alpha Pre-release

Known:

- Major feature development is completed
- CI testing passed
- Small-scale testing passed

Unknown:

- New feature may be unstable
- Regression may happen on existing features
- Large-scale testing may have issues
- May or may not be compatible

### Beta Pre-release

Known:

- New feature is stable
- No regression
- Large-scale testing passed
- Compatibility is confirmed

Unknown:

- Some undiscovered bug

### Release

Known:

- Existing issues resolved
- Documentation updated

Unknown:

- Security vulnerability

## Branching

### Master Branch

Master branch always has the most active and latest version.

### Release Branch

There could be multiple release branches in the pattern of `release-<major>.<minor>`, such as `release-4.2`.

The release branch always has the latest stable version for a certain minor version. However, some release versions may have already entered end-of-life, and the branches for them are only for archiving purpose.

## Tagging

### Git Tagging

The git tags consist of letter `v` with the version string, such as `v4.2.0` and `v4.2.0-alpha.0`.

### Docker Image Tagging

Two variants of images are available in [Docker Hub](https://hub.docker.com/r/zilliqa/zilliqa).

- CPU mining docker image: same as [git tagging](#git-tagging), such as `v4.2.0` or `v4.2.0-alpha.0`.
- CUDA mining docker images: starting with [git tagging](#git-tagging) with `-cuda` suffix, such as `v4.2.0-cuda` or `v4.2.0-alpha.0-cuda`.

In addition, the `latest` tag always points to the latest master version of Zilliqa (CPU mining version).

---
title: Commands
---

## Overview

Executing `s3d` provides a CLI with commands menu to configure and communicate with the daemon, and the `s3d run` command that runs the daemon itself.

The CLI is self documenting, so use `s3d [command] --help`

The sections below provide some overview of available commands.

## Run

Running the daemon in the foreground is done by the run command as shown below. 

The run command typically runs indefinitely but can also be configured to exit after a certain amount of time or once a specific key is created in a bucket.

```sh
s3d run
```

## Config

The following commands set up the configuration for the daemon.

See the [Config](config.md) page for more details.

```sh
s3d init
s3d config view
s3d config set <key> <value>
s3d config --help
```

## Data workflow commands

The workflows shown below are provided by `s3d` in the spirit of the similar git commands.

These commands are provided to easily define a data flow from the edge platform to the data center. 

These commands typically take optional bucket and prefix/key arguments to restrict the operation to more specific part of the data to operate on. 

`s3d` can run these workflows in the background or as a trigger to an event (like putting a specific key to a bucket).

```sh
# Fetch bucket metadata from the hub bucket and store locally
# This includes the list of objects and their metadata, but excludes objects contents
s3d fetch

# Pull is like fetch but includes objects contents.
s3d pull

# Simple access to objects from the CLI
s3d get bucket/key > file
s3d put bucket/key < file
s3d ls bucket/key

# Show bucket changes vs. remote hub bucket.
s3d diff

# Show bucket and local store status (objects, size, etc).
s3d status

# Push bucket changes (not override newer objects on hub).
s3d push

# Remove old objects from local store (configure if to allow removing objects that were not pushed to hub).
s3d prune
```

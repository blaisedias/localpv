# Overview

Use loop devices - ref https://man7.org/linux/man-pages/man8/losetup.8.html

## Creating a local pv backing device
* create a backing file
  * Create the file
  * *truncate* the file to the desired size
* attach it to a loop device

### Input parameters:
* path to backing file ( from storage class or PVC )
* volume size ( from PVC )

### Idempotency considerations:
* if backing file is attached on a loop device
  * if the loop device is mounted to the target path -> success
  * if the loop device is not bind mounted -> success (in progress)
  * else fail
* if backing file exists -> fail ( do not overwrite existing files )
  * Does an optional setting in StorageClass to overwrite existing file make sense?

## Destroying a local PV backing device
### Idempotency considerations:
* if backing file exists
  * if backing file is attached to a loop device
    * detach file from loop device
  * if PVC reclaim policy is delete:
    * delete backing file
      * -> success 
  * else -> success
* else -> success

## CSI driver
Derive the CSI driver from existing Mayastor CSI driver - replacing nexus and removing unsupported capabilities like volume expansion.

## Repository
Derive from Mayastor Control Plane reposotory

## Testing
TBD

## Other
systemd sensitivity when mounting ?
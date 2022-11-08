# New Zilliqa Daemon Design

## Background

Use Zilliqa daemon as the control point of all kinds of nodes. It will take the responsibility to launch the Zilliqa process and other auxiliary processes. The daemon can be used by community members too, they don’t need to manually restart the Zilliqa process if the Zilliqa process crashed.

## Zilliqa Daemon Process Design Change

1. It needs to get all the input parameters for Zilliqa process, so it can use them to launch the Zilliqa process.
1. After daemon started, it will check if Zilliqa process started, if not, it will start the Zilliqa process. The old behavior is if the Zilliqa process is not started, it waits for Zilliqa process to start.
1. Save the launch parameters into log file so can check later.
1. Following scripts should be launched after zilliqa process launched:  
    (1). scripts/uplaodIncrDB.py (only in lookup-0)  
    (2). scripts/auto_back_up.py (only in lookup-0)

## Testnet Repo Change

1. The bootstrap procedure of launching zilliqa should be removed (in bootstrap.py, and template/init.py)
1. The launched process of uploadIncrDB.py/auto_back_up.py should be removed (in template/init.py)
1. The template of launch.sh and launch_docker.sh both need to change. They will launch Zilliqa daemon process instead of Zilliqa process

## Upgrade Change

1. Currently we used some hacky way for upgrading, that is, only replace the zilliqa image inside the pod. And yaml also need to be updated after all nodes upgrade are done.
1. In the new approach, we should re-implement the upgrading to pod-image based replacement, with updated persistence from other nodes (S3).
1. Recovery implementation keeps the same, while using daemon & SUSPEND_LAUNCH to re-launch ZIlliqa process.

## Verify the change

1. Launch a small scale testnet to check the status
1. Kill Zilliqa process in random nodes, see if it can rejoin the testnet.
1. Recover lookup, dsguard and shard node
1. Recover community testnet
1. Rolling upgrade

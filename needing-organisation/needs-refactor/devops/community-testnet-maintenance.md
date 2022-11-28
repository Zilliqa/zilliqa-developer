# Community Testnet Maintenance

This page contains information relevant to maintaining the small-scale testnet for community members. This testnet resides in a dedicated cluster in `CommunityTestnetBastion` and is usually named `communityvXXX`.

## Basic Steps for Recovering the Testnet

In the event of a stall or any issue with the testnet, the fastest way to fix it is to perform recovery.

1. Follow the usual recovery steps described in [Network Upgrade](network-upgrade.md). Additionally, you can double-check the `.bootstrap_command` string in the current testnet's folder
2. Once the new testnet is up, manually launch the `uploadIncrDB.py` script in `lookup-0` so all nodes can rejoin
3. Wait until all 40 nodes are up in the next DS epoch
4. Send test transactions to the testnet
5. Redirect the API to this new testnet
6. Redirect the existing monitoring tools to this new testnet. Refer to [Mainnet Monitoring Scripts](mainnet-monitoring-scripts.md)
   - At the minimum, we just need `monitor_blockchain` and `lookup_autorecover`
   - For both scripts, use the Slack webhook URL for the testnet-alert channel
   - For `lookup_autorecover`, make sure to first terminate the script's process for the existing stalled testnet, before launching a new one
   - For `lookup_autorecover`, you can set `DEBUG_MODE` to `True` inside the script, because the testnet runs too fast for recovery to work anyway
7. Push the new testnet folder to `testnet-park` repo
   - Don't forget to include the `.bootstrap_command` and `.bootstrap_git_commit` files
   - Don't forget to update the list of active testnets in the `testnet-park` README

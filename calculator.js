const { Zilliqa } = require("@zilliqa-js/zilliqa");
const API = "https://xcad-isolated-server.zilliqa.com/";
const CONTRACT_ADDRESS = "0xf36a52881ca53efea4a89f70d0f7fbd418036546";
const USER_ADDRESS = "0x6d501ddc7b62a25ffbd4ee4ae806fb26cc91895f";
const zilliqa = new Zilliqa(API);

async function calculator() {
  try {
    console.log("rewards calculator: ");

    // 1.0 get last_block_num from smart contract
    const last_block_num_result =
      await zilliqa.blockchain.getSmartContractSubState(
        CONTRACT_ADDRESS,
        "last_block_num"
      );
    const last_block_num = Number(
      last_block_num_result.result["last_block_num"]
    );

    // 1.1 get current block from block chain
    // notice that isolated server does not support this api
    // const info_result = await zilliqa.blockchain.getLatestTxBlock();
    // const current_block = Number(info_result.result.header.BlockNum);
    current_block = Number(11547);

    // 1.3 get total_stake_per_cycle
    const total_stake_per_cycle_result =
      await zilliqa.blockchain.getSmartContractSubState(
        CONTRACT_ADDRESS,
        "total_stake_per_cycle"
      );
    const total_stake_per_cycle_remote =
      total_stake_per_cycle_result.result["total_stake_per_cycle"];
    const total_stake_per_cycle_map = {};
    for (const element in total_stake_per_cycle_remote) {
      total_stake_per_cycle_map[Number(element)] =
        total_stake_per_cycle_remote[element];
    }

    // 1.4 get blocks_per_cycle from contract init parameter
    const init_param_result = await zilliqa.blockchain.getSmartContractInit(
      CONTRACT_ADDRESS
    );
    const block_per_cycle = await get_blocks_per_cycle(
      init_param_result.result
    );

    // 1.5 get total stake from contract
    const total_stake_result =
      await zilliqa.blockchain.getSmartContractSubState(
        CONTRACT_ADDRESS,
        "total_stake"
      );
    const total_stake = Number(total_stake_result.result["total_stake"]);

    // 1.6 get last_cycle from contract
    const last_cycle_result = await zilliqa.blockchain.getSmartContractSubState(
      CONTRACT_ADDRESS,
      "last_cycle"
    );
    var last_cycle = Number(last_cycle_result.result["last_cycle"]);

    console.table({
      total_stake_per_cycle_map: total_stake_per_cycle_map,
    });

    // 1.5 extend local total_stake_per_cycle_map
    const cycle_to_increase =
      (current_block - last_block_num) / block_per_cycle;
    for (let i = 1; i <= cycle_to_increase; i++) {
      total_stake_per_cycle_map[last_cycle + i] = total_stake;
    }

    last_cycle += cycle_to_increase;

    console.table({
      total_stake_per_cycle_map: total_stake_per_cycle_map,
    });

    // 1.6 get last withdraw cycle from contract
    const last_withdraw_cycle_result =
      await zilliqa.blockchain.getSmartContractSubState(
        CONTRACT_ADDRESS,
        "last_withdraw_cycle"
      );
    const last_withdraw_cycle_map =
      last_withdraw_cycle_result.result["last_withdraw_cycle"];
    console.table({
      last_withdraw_cycle_map: last_withdraw_cycle_map,
    });
    const last_withdraw_cycle = await get_last_withdraw_cycle(
      USER_ADDRESS,
      last_withdraw_cycle_map
    );

    // 1.7 get stakers_stake_per_cycle map from contract
    const stakers_stake_per_cycle_result =
      await zilliqa.blockchain.getSmartContractSubState(
        CONTRACT_ADDRESS,
        "stakers_stake_per_cycle"
      );
    const stakers_stake_per_cycle =
      stakers_stake_per_cycle_result.result["stakers_stake_per_cycle"];
    const stakers_stake_per_cycle_local = {};
    stakers_stake_per_cycle_local[USER_ADDRESS] = {};
    stakers_stake_per_cycle_local[USER_ADDRESS][last_withdraw_cycle] = Number(
      stakers_stake_per_cycle[USER_ADDRESS][last_withdraw_cycle.toString()]
    );

    // 1.8 get stakers_bal from contract
    const stakers_bal_result =
      await zilliqa.blockchain.getSmartContractSubState(
        CONTRACT_ADDRESS,
        "stakers_bal"
      );
    const stakers_bal = stakers_bal_result.result["stakers_bal"];

    // 1.9 get rewards_per_cycle from contract
    const rewards_per_cycle_result =
      await zilliqa.blockchain.getSmartContractSubState(
        CONTRACT_ADDRESS,
        "rewards_per_cycle"
      );
    const rewards_per_cycle =
      rewards_per_cycle_result.result["rewards_per_cycle"];

    console.table({
      last_block_num: last_block_num,
      current_block: current_block,
      block_per_cycle: block_per_cycle,
      total_stake: total_stake,
      last_cycle: last_cycle,
      last_withdraw_cycle: last_withdraw_cycle,
    });

    const total_rewards_map = {};

    for (let i = last_withdraw_cycle + 1; i < last_cycle; i++) {
      const last_staker_bal = await get_last_staker_bal(
        stakers_stake_per_cycle_local,
        USER_ADDRESS,
        i - 1
      );
      delete stakers_stake_per_cycle_local[USER_ADDRESS][(i - 1).toString()];
      const deposit_this_cycle = await get_deposit_this_cycle(
        stakers_bal,
        USER_ADDRESS,
        i
      );
      const stakers_total_delegate_this_cycle =
        last_staker_bal + deposit_this_cycle;
      stakers_stake_per_cycle_local[USER_ADDRESS][i.toString()] =
        stakers_total_delegate_this_cycle;

      const total_delegate_this_cycle = total_stake_per_cycle_map[i];
      console.table({
        cycle: i,
        stakers_total_delegate_this_cycle: stakers_total_delegate_this_cycle,
        total_delegate_this_cycle: total_delegate_this_cycle,
      });

      for (const element in rewards_per_cycle) {
        const total_rewards = rewards_per_cycle[element];
        const rewards =
          (stakers_total_delegate_this_cycle * total_rewards) /
          total_delegate_this_cycle;
        const existing_rewards = await get_existing_rewards(
          total_rewards_map,
          element
        );
        const accumulated = existing_rewards + rewards;
        total_rewards_map[element] = accumulated;
      }
    }

    console.table(total_rewards_map);
  } catch (e) {
    console.log(e);
  }
}

async function get_existing_rewards(total_rewards_map, token) {
  var res = Number(0);
  const existing = total_rewards_map[token];
  if (existing !== undefined) {
    res = existing;
  }
  return res;
}

async function get_deposit_this_cycle(stakers_bal, user, this_cycle) {
  var res = Number(0);
  const deposit = stakers_bal[user][this_cycle];
  if (deposit !== undefined) {
    bal_res = Number(deposit);
  }
  return res;
}

async function get_last_staker_bal(
  stakers_stake_per_cycle_local,
  user,
  last_cycle
) {
  var bal_res = Number(0);
  const last_bal = stakers_stake_per_cycle_local[user][last_cycle];
  if (last_bal !== undefined) {
    bal_res = last_bal;
  }
  return bal_res;
}

async function get_blocks_per_cycle(init_param) {
  var res;
  init_param.forEach((element) => {
    if (element["vname"] === "blocks_per_cycle") {
      res = Number(element["value"]);
    }
  });
  return res;
}

async function get_last_withdraw_cycle(user, last_withdraw_cycle_map) {
  const last_withdraw = last_withdraw_cycle_map[user];
  if (last_withdraw === undefined) {
    return Number(0);
  }
  return Number(last_withdraw);
}

calculator();

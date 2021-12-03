# Staking Contract for ZRC-2 Tokens

The [contract](./staking.scilla) code in this repository is designed to implement the following specification:

1. The system has two participants namely stakers and admin.
2. Stakers can deposit a ZRC-2 token of their choice and earn the same or another ZRC-2 token. Rewards could potentially be in multiple tokens and is available for claim at the end of every reward cycle which is set to be 1 day or 2,500 blocks. 
3. Reward amount and the tokens that users earn are decided by the admin at the contract deployment time. The admin also specifies the duration of the reward campaign (e.g., 60 days). For example, the admin would specify that users staking a certain Token A will earn Token B and Token C for a total of 10 reward cycles and the total reward amount for each token is 1,000. I.e., for each reward cycle, 100 Token B and 100 Token C are to be disbursed among the stakers during that cycle.
4. Reward earned by a given staker for a given reward cycle is proportional to his stake at the end of the cycle. 
5. Stakers need to lock their tokens for a specified number of cycles to earn rewards. In other words, once a staker has deposited funds, he cannot take his funds out for 7 cycles.
6. In case a staker wants to take his funds within 7 days, he has to pay a penalty of 10% to the system. 10% is on the staked tokens being withdrawn. These tokens can be claimed by the admin at any point of time.
7. Safety features should be in the contract that allows the admin to withdraw the rewards and the stake in case of emergency.

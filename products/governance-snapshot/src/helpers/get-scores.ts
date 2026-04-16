import { fromBech32Address } from '@zilliqa-js/zilliqa';
import { validation } from "@zilliqa-js/util";
import { zilliqa } from '@/helpers/zilliqa';

export const _strategies = {
  'zrc2-balance-of': {
    author: 'zilpay',
    key: 'zrc2-balance-of',
    version: '0.0.1',
    strategy: async function(_provider: any, addresses: string[], options: any) {
      if (Array.isArray(addresses) && addresses.length === 0) {
        return [];
      }

      const balances = await Promise.all(
        addresses.map(async address => {
          address = validation.isBech32(address)
            ? fromBech32Address(address).toLowerCase()
            : String(address).toLowerCase();

          try {
            const proposal = window['proposal'];

            if (proposal && proposal['balances']) {
              const amount = proposal['balances'][address];

              if (!amount) {
                return [address, '0'];
              }

              return [
                address,
                Number(amount) / Math.pow(10, Number(options.decimals))
              ];
            }
          } catch (err) {
            return [address, '0'];
          }

          const {
            result
          } = await zilliqa.blockchain.getSmartContractSubState(
            options.address,
            'balances',
            [address]
          );

          if (result && result.balances && result.balances[address]) {
            const amount = result.balances[address];
            return [
              address,
              Number(amount) / Math.pow(10, Number(options.decimals))
            ];
          }

          return [address, '0'];
        })
      );

      return Object.fromEntries(balances);
    }
  }
};

export async function getScores(
  strategies: any[],
  _provider?: any,
  addresses: string[] = []
) {
  return await Promise.all(
    strategies.map(strategy => {
      return _strategies[strategy.name].strategy(
        _provider,
        addresses,
        strategy.params
      );
    })
  );
}

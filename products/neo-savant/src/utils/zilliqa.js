// Copyright (C) 2020 Zilliqa

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https:www.gnu.org/licenses/>.

import { schnorr, getAddressFromPrivateKey } from "@zilliqa-js/crypto";

const generateZilliqaAccount = async () => {
  const privateKey = await schnorr.generatePrivateKey();

  const address = await getAddressFromPrivateKey(privateKey);
  return {
    address,
    privateKey,
  };
};

const generateMultipleZilliqaAccounts = async (count) => {
  let accounts = [];
  for (let i = 1; i <= count; i++) {
    const acc = await generateZilliqaAccount();

    await accounts.push(acc);
  }

  return accounts;
};

export { generateZilliqaAccount, generateMultipleZilliqaAccounts };

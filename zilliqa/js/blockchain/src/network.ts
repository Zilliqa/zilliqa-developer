//  Copyright (C) 2018 Zilliqa
//
//  This file is part of zilliqa-js
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.

import {
  Provider,
  RPCMethod,
  RPCResponse,
  ZilliqaModule,
} from "@zilliqa-js/core";
import { Wallet } from "@zilliqa-js/account";

export class Network implements ZilliqaModule {
  provider: Provider;
  signer: Wallet;

  constructor(provider: Provider, signer: Wallet) {
    this.provider = provider;
    this.signer = signer;
  }

  getNetworkId(): Promise<RPCResponse<string, string>> {
    return this.provider.send(RPCMethod.GetNetworkId);
  }

  getVersion(): Promise<RPCResponse<string, string>> {
    return this.provider.send(RPCMethod.GetVersion);
  }

  getNodeType(): Promise<RPCResponse<string, string>> {
    return this.provider.send(RPCMethod.GetNodeType);
  }

  getNumPeers(): Promise<RPCResponse<number, string>> {
    return this.provider.send(RPCMethod.GetNumPeers);
  }
}

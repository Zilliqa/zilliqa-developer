package com.zilliqa.laksaj.account;

import com.zilliqa.laksaj.jsonrpc.HttpProvider;
import com.zilliqa.laksaj.transaction.Transaction;
import com.zilliqa.laksaj.transaction.TransactionFactory;
import org.junit.Test;
import org.junit.Rule;
import org.junit.rules.Timeout;

import java.util.ArrayList;
import java.util.List;

import static com.zilliqa.laksaj.account.Wallet.pack;

public class WalletTest {
  // Use this with "mitmweb --mode reverse:https://dev-api.zilliqa.com -p 8082" to debug transactions.
  //static final String ENDPOINT_URL = "http://localhost:8082";
  static final String ENDPOINT_URL = "https://dev-api.zilliqa.com/";

  @Test(timeout=180000)
    public void sendTransactionTest() throws Exception {
        Wallet wallet = new Wallet();
        String privateKey = "82453a2882829fcc959a07c86b6f36ac613c18f2591390b669a3449e90412e41";
        // Populate the wallet with an account
        String address = wallet.addByPrivateKey(privateKey);
        wallet.addByPrivateKey(privateKey);

        HttpProvider provider = new HttpProvider(ENDPOINT_URL);
        wallet.setProvider(provider);
        //get balance
        HttpProvider.BalanceResult balanceResult = provider.getBalance(address).getResult();
        Integer nonce = Integer.parseInt(balanceResult.getNonce());

        //construct non-contract transaction
        Transaction transaction = Transaction.builder()
                                  .version(String.valueOf(pack(333, 1)))
                                  //                .toAddr("24A4zoHhcP4PGia5e5aCnEbq4fQw")
                                  //                .toAddr("0x4baf5fada8e5db92c3d3242618c5b47133ae003c".toLowerCase())
                                  //                .toAddr("4BAF5faDA8e5Db92C3d3242618c5B47133AE003C")
                                  .toAddr("zil16jrfrs8vfdtc74yzhyy83je4s4c5sqrcasjlc4")
                                  .senderPubKey("0344b122da41bbd5b53c0c6d7416d135b6d5c88e5e95ff4d22f9cf9ca45c368a1c")
                                  .amount("10000101")
                                  .nonce(Integer.toString(nonce + 1))
                                  .gasPrice("2000000000")
                                  .gasLimit("50")
                                  .code("")
                                  .data("")
                                  .provider(new HttpProvider(ENDPOINT_URL))
                                  .build();

        //sign transaction
        transaction = wallet.sign(transaction);

        //broadcast transaction
        HttpProvider.CreateTxResult result = TransactionFactory.createTransaction(transaction);
        transaction.confirm(result.getTranID(), 100, 3);
    }

    @Test(timeout=180000)
    public void sendTransactionsTest() throws Exception {
        Wallet wallet = new Wallet();
        String privateKey = "92e8138fe112cdcd7b48e27f64c7e4ff533caef7317574a1a337c93689785140";
        // Populate the wallet with an account
        String address = wallet.addByPrivateKey(privateKey);
        wallet.addByPrivateKey(privateKey);

        HttpProvider provider = new HttpProvider(ENDPOINT_URL);
        wallet.setProvider(provider);
        //get balance
        HttpProvider.BalanceResult balanceResult = provider.getBalance(address).getResult();
        Integer nonce = Integer.parseInt(balanceResult.getNonce());

        //construct non-contract transactions
        List<Transaction> transactions = new ArrayList<>();
        for (int i =0;i < 16; ++i) {
          Transaction tx = Transaction.builder()
                           .version(String.valueOf(pack(333, 1)))
                           .toAddr("zil16jrfrs8vfdtc74yzhyy83je4s4c5sqrcasjlc4")
                           .senderPubKey("035e37f311c6fa25829d29d1055256912719586971b11499d904726ec6f0d9de5b")
                           .amount(Integer.toString(10001000 + i))
                           .nonce(Integer.toString(nonce + 1+i))
                           .gasPrice("2000000000")
                           .gasLimit("50")
                           .code("")
                           .data("")
                           .provider(provider)
                           .build();
          transactions.add(tx);
        }

        wallet.batchSign(transactions);
        provider.createTransactions(transactions);
        TransactionFactory.batchConfirm(transactions, 100, 3);

        // do some post check, e.g check errors
        for (int i = 0; i < transactions.size(); i++) {
            // we expected transaction 3 is failed because of gas fee setting
            // (whose to address is zil1n0lvw9dxh4jcljmzkruvexl69t08zs62ds9ats)
            if (transactions.get(i).getToAddr().equals("0x9BFEC715a6bD658fCb62B0f8cc9BFa2ADE71434A")) {
                System.out.println(transactions.get(i).getInfo());
            }
        }

    }
}
